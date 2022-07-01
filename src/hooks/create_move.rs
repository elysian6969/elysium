use crate::{state, Entity};
use elysium_math::Vec3;
use elysium_sdk::convar::Vars;
use elysium_sdk::entity::{Networkable, ObserverMode, Renderable};
use elysium_sdk::{Command, EntityList, HitGroup};

const IN_FORWARD: i32 = 1 << 3;
const IN_BACKWARD: i32 = 1 << 4;

// yes.
const IN_LEFTWARD: i32 = 1 << 7;
const IN_RIGHTWARD: i32 = 1 << 8;

const IN_ATTACK: i32 = 1 << 0;
const IN_BULLRUSH: i32 = 1 << 22;
const IN_JUMP: i32 = 1 << 1;

const ON_GROUND: i32 = 1 << 0;

#[inline]
fn fix_movement(command: &mut Command, wish_angle: Vec3) {
    let (mut wish_forward, mut wish_right, _wish_up) = wish_angle.angle_vector();
    let (mut curr_forward, mut curr_right, _curr_up) = command.view_angle.angle_vector();

    wish_forward.z = 0.0;
    wish_right.z = 0.0;
    curr_forward.z = 0.0;
    curr_right.z = 0.0;

    fn normalize(vec: &mut Vec3) {
        let radius = (vec.x * vec.x + vec.y * vec.y + vec.z * vec.z).sqrt();
        let iradius = 1.0 / (radius + f32::EPSILON);

        vec.x *= iradius;
        vec.y *= iradius;
        vec.z *= iradius;
    }

    normalize(&mut wish_forward);
    normalize(&mut wish_right);
    normalize(&mut curr_forward);
    normalize(&mut curr_right);

    fn get_dir(movement: Vec3, forward: Vec3, right: Vec3) -> Vec3 {
        let x = forward.x * movement.x + right.x * movement.y;
        let y = forward.y * movement.x + right.y * movement.y;

        Vec3::from_xy(x, y)
    }

    let wish_dir = get_dir(command.movement, wish_forward, wish_right);
    let curr_dir = get_dir(command.movement, curr_forward, curr_right);

    if wish_dir != curr_dir {
        let denom = curr_right.y * curr_forward.x - curr_right.x * curr_forward.y;

        command.movement.x = (wish_dir.x * curr_right.y - wish_dir.y * curr_right.x) / denom;
        command.movement.y = (wish_dir.y * curr_forward.x - wish_dir.x * curr_forward.y) / denom;
    }
}

// TODO: find out how the fuck to fix the legs being spaz
//
// also seems like left/right doesnt work
#[inline]
fn leg_animation_walk(command: &mut Command) {
    command.state ^= IN_FORWARD | IN_BACKWARD | IN_RIGHTWARD | IN_LEFTWARD;
}

#[allow(dead_code)]
#[inline]
fn scale_damage(
    entity: &Entity,
    group: HitGroup,
    weapon_armor_ratio: f32,
    mut base_damage: f32,
) -> f32 {
    base_damage *= group.damage_modifier();

    if entity.armor() > 0 {
        if group.is_head() && entity.has_helmet() {
            base_damage *= weapon_armor_ratio * 0.5;
        }
    }

    base_damage
}

#[allow(dead_code)]
fn calculate_angle(src: Vec3, dst: Vec3) -> Vec3 {
    println!("src = {src:?}, dst = {dst:?}");

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

#[allow(unused_variables)]
#[inline]
unsafe fn do_create_move(command: &mut Command, local: &Entity, send_packet: &mut bool) {
    let vars = &*state::vars().cast::<Vars>();
    let local_vars = state::Local::get();

    // can you dont when on ladder or in noclip
    if matches!(local.move_kind(), 8 | 9) {
        return;
    }

    if local_vars.was_attacking {
        command.state &= !IN_ATTACK;
    }

    let do_attack = (command.state & IN_ATTACK) != 0;
    let do_jump = (command.state & IN_JUMP) != 0;
    let on_ground = (local.flags() & ON_GROUND) != 0;

    local_vars.was_attacking = do_attack;
    local_vars.was_on_ground = on_ground;

    if do_jump {
        if !on_ground && !local_vars.was_on_ground {
            command.state &= !IN_JUMP;
        }
    }

    let side = if command.command % 3 != 0 { 1.0 } else { -1.0 };

    if (local.flags() & ON_GROUND) == 0 {
        let velocity = local.velocity();
        let magnitude = velocity.magnitude2d();
        let ideal_strafe = (15.0 / magnitude).atan().to_degrees().clamp(0.0, 90.0);
        let mut wish_angle = command.view_angle;
        let strafe_dir = command.movement.to_dir();
        let strafe_dir_yaw_offset = strafe_dir.y.atan2(strafe_dir.x).to_degrees();

        wish_angle.y -= strafe_dir_yaw_offset;

        let mut wish_angle = wish_angle.sanitize_angle();
        let yaw_delta = libm::remainderf(wish_angle.y - local_vars.old_yaw, 360.0);
        let abs_yaw_delta = yaw_delta.abs();

        local_vars.old_yaw = wish_angle.y;

        let horizontal_speed = vars.horizontal_speed.read();

        if abs_yaw_delta <= ideal_strafe || abs_yaw_delta >= 30.0 {
            let velocity_dir = Vec3::vector_angle(velocity);
            let velocity_yaw_delta = libm::remainderf(wish_angle.y - velocity_dir.y, 360.0);
            let retrack = (30.0 / magnitude).atan().to_degrees().clamp(0.0, 90.0) * 2.0;

            if velocity_yaw_delta <= retrack || magnitude <= 15.0 {
                if -retrack <= velocity_yaw_delta || magnitude <= 15.0 {
                    wish_angle.y += side * ideal_strafe;
                    command.movement.y = horizontal_speed * side;
                } else {
                    wish_angle.y = velocity_dir.y - retrack;
                    command.movement.y = horizontal_speed;
                }
            } else {
                wish_angle.y = velocity_dir.y + retrack;
                command.movement.y = -horizontal_speed;
            }
        } else if yaw_delta > 0.0 {
            command.movement.y = -horizontal_speed;
        } else if yaw_delta < 0.0 {
            command.movement.y = horizontal_speed
        }

        command.movement.x = 0.0;

        fix_movement(command, wish_angle);
    }

    // 89.0 = down, -89.0 = up
    let pitch = 89.0;

    // 180.0 for backwards
    let yaw_base = 180.0;

    // roll base
    let roll_base = 0.0;

    // how much to jitter yaw
    let jitter_yaw = 12.0;

    // how much to jitter roll
    let jitter_roll = 50.0;

    // note: remember, desync isnt static, nor can it always be 58.0;
    let desync = 58.0;

    command.view_angle.x = pitch;
    command.view_angle.y += yaw_base - desync + (jitter_yaw * side);
    command.view_angle.z += roll_base + jitter_roll * side;

    if *send_packet {
        command.view_angle.y += desync;
    } else {
        command.view_angle.y += 120.0;
    }

    if do_attack {
        *send_packet = true;
        command.view_angle = *state::view_angle();
    }

    command.view_angle = command.view_angle.sanitize_angle();
    command.state |= IN_BULLRUSH;

    use elysium_sdk::{Engine, EntityList, Frame, Globals, Input, InputSystem};

    let engine = &*state::engine().cast::<Engine>();
    let entity_list = &*state::entity_list().cast::<EntityList>();
    let globals = &*state::globals().cast::<Globals>();
    let input = &mut *state::input().as_mut().cast::<Input>();

    let players = &mut *state::players();
    let renderable = &*<*const Entity>::byte_add(local, 8).cast::<Renderable>();
    let networkable = <*const Entity>::byte_add(local, 16).cast::<Networkable>();
    println!("{networkable:?}");
    let networkable = &*networkable;
    let local_index = networkable.index();

    // iterate player list
    for index in entity_list.player_range() {
        // skip local
        if index == local_index {
            continue;
        }

        let bones = &mut players[index as usize - 1].bones;
        let entity = entity_list.entity(index);

        // skip nonexistent
        if entity.is_null() {
            *bones = providence_model::Bones::zero();
            continue;
        }

        let entity = &*entity.cast::<Entity>();
        let renderable = &*<*const Entity>::byte_add(entity, 8).cast::<Renderable>();
        let networkable = &*<*const Entity>::byte_add(entity, 16).cast::<Networkable>();

        // skip dormant
        if networkable.is_dormant() {
            *bones = providence_model::Bones::zero();
            continue;
        }

        renderable.setup_bones(&mut bones[0..128], 0x00000100, globals.current_time);
        renderable.setup_bones(&mut bones[0..128], 0x000FFF00, globals.current_time);

        let eye_origin = local.eye_origin();
        let bone_origin = bones.get_origin(8).unwrap_unchecked();

        command.view_angle = calculate_angle(eye_origin, bone_origin);
    }

    fix_movement(command, *state::view_angle());
    leg_animation_walk(command);
}

/// `CreateMove` hook.
pub unsafe extern "C" fn create_move(
    this: *const u8,
    input_sample_time: f32,
    command: *mut u8,
) -> bool {
    state::hooks::create_move(this, input_sample_time, command);

    let command = &mut *command.cast::<Command>();
    let local_vars = state::Local::get();

    if command.tick_count == 0 || local_vars.player.is_null() {
        return false;
    }

    let local = &*local_vars.player.cast::<Entity>();

    // can you dont when spectatng
    if local.observer_mode() != ObserverMode::None {
        return false;
    }

    let rbp: *mut *mut bool;

    core::arch::asm!("mov {}, rbp", out(reg) rbp, options(nostack));

    let send_packet = &mut *(*rbp).sub(24);

    *send_packet = command.command % 2 != 0;

    do_create_move(command, local, send_packet);

    if *send_packet {
        local_vars.view_angle = command.view_angle;
    }

    false
}
