use crate::{state, Entity};
use core::ptr;
use elysium_math::Vec3;
use elysium_sdk::trace::{Contents, Filter, Mask, Ray, Summary, SurfaceFlags, TraceKind};
use elysium_sdk::Trace;

const STEP_F32: f32 = 4.0;
const STEP_VEC: Vec3 = Vec3::splat(STEP_F32);

struct SkipEntity {
    entity: *const Entity,
}

impl Filter for SkipEntity {
    fn should_hit(&self, entity: *const u8, mask: i32) -> bool {
        self.entity == entity.cast()
    }

    fn trace_kind(&self) -> TraceKind {
        TraceKind::Everything
    }
}

fn skip(entity: *const Entity) -> SkipEntity {
    SkipEntity { entity }
}

pub struct ShotData {
    pub source: Vec3,
    pub enter_summary: Summary,
    pub direction: Vec3,
    pub filter: *const Entity,
    pub trace_length: f32,
    pub trace_length_remaining: f32,
    pub current_damage: f32,
    pub penetrate_count: i32,
}

const IDK_MASK: u32 = Mask::SHOT_HULL.0 | Contents::HITBOX.0;

fn trace_to_exit(
    start: Vec3,
    direction: Vec3,
    enter_summary: &Summary,
    exit_summary: &mut Summary,
    end: &mut Vec3,
) -> bool {
    let trace = &*state::trace().cast::<Trace>();
    let iter = (0..=90).step_by(4).map(f32::from);

    for distance in iter {
        *end = start + direction * Vec3::splat(distance);

        let contents = trace.point_contents(
            /* position: Vec3 */ *end,
            /* contents: u32 */ IDK_MASK,
            /* entities: *const *const u8 */ ptr::null(),
        );

        let has_mask_shot_hull = contents & !Mask::SHOT_HULL.0 != 0;
        let has_hitbox = contents & !Contents::HITBOX.0 != 0;

        if has_mask_shot_hull && has_hitbox {
            continue;
        }

        let new_end = *end - (direction * STEP_VEC);

        trace.trace(
            /* ray: Ray */ Ray::new(*end, new_end),
            /* contents: u32 */ IDK_MASK,
            /* filter */ skip(local),
        );

        if exit_summary.start_within_solid && exit_summary.has_flag(SurfaceFlags::HITBOX) {
            trace.trace(
                Ray::new(*end, start),
                IDK_MASK,
                skip(exit_summary.entity.cast()),
            );

            if (exit_summary.fraction <= 1.0 || exit_summary.within_solid)
                && !exit_summary.start_within_solid
            {
                *end = exit_summary.hit_pos;
                return true;
            }

            continue;
        }

        if !(exit_summary.fraction <= 1.0
            || exit_summary.within_solid
            || exit_summary.start_within_solid)
            || exit_summary.start_within_solid
        {
            if exit_summary.entity.is_null() {
                return true;
            }

            continue;
        }

        if exit_summary.has_flag(SurfaceFlags::NO_DRAW) {
            continue;
        }

        if let Some(plane) = exit_summary.plane() {
            if plane.normal.dot(direction) <= 1.0 {
                let fraction = exit_summary.fraction * STEP_F32;

                *end = *end - (direction * Vec3::splat(fraction));

                return true;
            }
        }
    }

    false
}

impl ShotData {
    pub fn new() -> Self {
        Self {
            source: Vec3::zero(),
            enter_summary: Summary::new(),
            direction: Vec3::zero(),
            filter: ptr::null(),
            trace_length: 0.0,
            trace_length_remaining: 0.0,
            current_damage: 0.0,
            penetrate_count: 0,
        }
    }

    pub fn handle_bullet_penetration(&mut self, weapon: &Entity) -> bool {
        let physics = &*state::physics().cast::<Physics>();

        let surface = match physics.query(self.enter_summary.surface.properties as i32) {
            Some(surface) => surface,
            None => return true,
        };

        let enter_material = surface.properties.material;
        let enter_penetration_modifier = surface.properties.penetration_modifier;

        self.trace_length += self.trace_length_remaining * self.enter_summary.fraction;
        self.current_damage *= weapon.range_modifier().powf(self.trace_length * 0.002);

        if self.trace_length > 3000.0 || enter_penetration_modifier < 0.1 {
            self.penetrate_count = 0;
        }

        if self.penetrate_count <= 0 {
            return false;
        }

        let mut end = Vec3::zero();
        let mut exit_summary = Summary::new();

        if !trace_to_exit(
            /* start */ self.enter_summary.hit_pos,
            /* direction */ self.direction,
            /* enter_summary */ &self.enter_summary,
            /* exit_summary */ &mut exit_summary,
            /* end_pos */ &mut end,
        ) {
            return false;
        }

        let surface = match global.physics().query(exit_summary.surface.properties as _) {
            Some(surface) => surface,
            None => return true,
        };

        let exit_material = surface.properties.material;
        let exit_penetration_modifier = surface.properties.penetration_modifier;
        let mut final_damage_modifier: f32 = 0.16;
        let mut combined_penetration_modifier: f32 = 0.0;

        if self.enter_summary.contents.has_grate() || matches!(enter_material, 71 | 89) {
            final_damage_modifier = 0.05;
            combined_penetration_modifier = 3.0;
        } else {
            combined_penetration_modifier =
                (enter_penetration_modifier + exit_penetration_modifier) * 0.5;
        }

        if enter_material == exit_material {
            if matches!(exit_material, 85 | 87) {
                combined_penetration_modifier = 3.0;
            } else if exit_material == 76 {
                combined_penetration_modifier = 2.0;
            }
        }

        let v34 = f32::max(0.0, 1.0 / combined_penetration_modifier);
        let v35 = self.current_damage * final_damage_modifier
            + v34 * 3.0 * f32::max(0.0, (3.0 / weapon.penetration()) * 1.25);

        let mut thickness = (exit_summary.hit_pos - self.enter_summary.hit_pos).magnitude();

        thickness = (thickness * thickness * v34) / 24.0;

        let lost_damage = f32::max(0.0, v35 + thickness);

        if lost_damage > self.current_damage {
            return false;
        }

        if lost_damage >= 0.0 {
            self.current_damage -= lost_damage;
        }

        if self.current_damage < 1.0 {
            return false;
        }

        self.source = exit_summary.hit_pos;
        self.penetrate_count -= 1;

        // cant shoot through this
        true
    }

    pub fn simulate_shot(&mut self, local: &Entity, weapon: &Entity) -> bool {
        let trace = &*state::trace().cast::<Trace>();
        let weapon_damage = weapon.damage();
        let weapon_range = weapon.range();
        let weapon_range_modifier = weapon.range_modifier();

        self.penetrate_count = 4;
        self.trace_length = 0.0;
        self.current_damage = weapon_damage;

        while self.penetrate_count > 0 && self.current_damage >= 1.0 {
            self.trace_length_remaining = weapon_range - self.trace_length;

            let end = self.source + self.direction * Vec3::splat(self.trace_length_remaining);
            let new_end = end + self.direction * STEP_VEC;

            trace.trace(Ray::new(self.source, end), Mask::SHOT.0, skip(local));

            trace.trace(
                Ray::new(self.source, new_end),
                Mask::SHOT.0,
                skip(self.filter),
            );

            trace.trace(Ray::new(self.source, new_end), Mask::SHOT.0, skip(local));

            if self.enter_summary.fraction == 1.0 {
                break;
            }

            if self.enter_summary.hit_group.is_hit() {
                return true;
            }

            break;
        }

        false
    }
}

fn calculate_angle(src: Vec3, dst: Vec3) -> Vec3 {
    let delta = src - dst;
    let hypot = (delta.x * delta.x + delta.y * delta.y).sqrt();

    let x = (delta.z / hypot).atan().to_degrees();
    let mut y = (delta.y / delta.x).atan().to_degrees();
    let z = 0.0;

    if delta.x >= 0.0 {
        y += 180.0;
    }

    Vec3::from_xyz(x, y, z)
}

fn angle_vector(angle: &Vec3, forward: &mut Vec3) {
    let x = angle.x.to_radians();
    let y = angle.y.to_radians();

    let (x_sin, x_cos) = x.sin_cos();
    let (y_sin, y_cos) = y.sin_cos();

    forward.x = x_cos * y_cos;
    forward.y = x_cos * y_sin;
    forward.z = -x_sin;
}

fn get_damage(local: &Entity, weapon: &Entity, destination: Vec3) -> f32 {
    let mut shot_data = ShotData::new();

    shot_data.source = local.eye_origin();
    shot_data.filter = local;

    let angle = calculate_angle(shot_data.source, destination);

    angle_vector(&angle, &mut shot_data.direction);

    shot_data.direction = shot_data.direction.normalize();

    if shot_data.simulate_shot(local, weapon) {
        shot_data.current_damage
    } else {
        -1.0
    }
}
