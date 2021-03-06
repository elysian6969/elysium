use crate::{Networked, State};
use cake::ffi::vtable;
use elysium_math::{Matrix3x4, Vec3};
use elysium_sdk::entity::{Networkable, ObserverMode, Renderable};
use elysium_sdk::{object_validate, vtable_validate};

#[repr(C)]
struct VTable {
    _pad0: vtable::Pad<12>,
    origin: unsafe extern "thiscall" fn(this: *const Entity) -> *const Vec3,
    _pad1: vtable::Pad<144>,
    is_player: unsafe extern "thiscall" fn(this: *const Entity) -> bool,
    _pad2: vtable::Pad<199>,
    observer_mode: unsafe extern "thiscall" fn(this: *const Entity) -> ObserverMode,
}

vtable_validate! {
    origin => 12,
    is_player => 157,
    observer_mode => 357,
}

#[repr(C)]
pub struct Entity {
    vtable: &'static VTable,
    pub renderable: Renderable,
    pub networkable: Networkable,
}

object_validate! {
    Entity;
    vtable => 0,
    renderable => 8,
    networkable => 16,
}

impl Entity {
    /// for whatever reason, things get optimized weirdly, and result in segmentation faults
    #[inline(never)]
    pub fn renderable<'a>(entity: *const Entity) -> &'a Renderable {
        unsafe { &*entity.byte_add(8).cast::<Renderable>() }
    }

    /// for whatever reason, things get optimized weirdly, and result in segmentation faults
    #[inline(never)]
    pub fn networkable<'a>(entity: *const Entity) -> &'a Networkable {
        unsafe { &*entity.byte_add(16).cast::<Networkable>() }
    }

    /// the entity's class
    #[inline]
    pub fn client_class(&self) -> *const u8 {
        Self::networkable(self).client_class()
    }

    /// is the entity dormant
    #[inline]
    pub fn is_dormant(&self) -> bool {
        Self::networkable(self).is_dormant()
    }

    /// the entity's index
    #[inline]
    pub fn index(&self) -> i32 {
        Self::networkable(self).index()
    }

    /// the entity's model
    #[inline]
    pub fn model(&self) -> *const u8 {
        Self::renderable(self).model()
    }

    /// setup bones
    #[inline]
    pub fn setup_bones(&self, bones: &mut [Matrix3x4], mask: i32, time: f32) -> bool {
        Self::renderable(self).setup_bones(bones, mask, time)
    }

    /// should draw?
    #[inline]
    pub fn should_draw(&self) -> bool {
        Self::renderable(self).should_draw()
    }

    #[inline]
    pub fn origin(&self) -> Vec3 {
        unsafe { *(self.vtable.origin)(self) }
    }

    #[inline]
    pub fn is_player(&self) -> bool {
        unsafe { (self.vtable.is_player)(self) }
    }

    #[inline]
    pub fn observer_mode(&self) -> ObserverMode {
        unsafe { (self.vtable.observer_mode)(self) }
    }

    /// networked variable
    #[inline]
    fn networked<T, F>(&self, f: F) -> &mut T
    where
        F: Fn(&Networked) -> usize,
    {
        unsafe {
            let this = (self as *const Self).cast::<u8>();
            let state = State::get();
            let offset = f(&state.networked);

            &mut *this.byte_add(offset).as_mut().cast()
        }
    }

    /// only for base_entitys
    #[inline]
    fn render_mode_address(&self) -> *const u8 {
        self.networked(|networked| networked.base_entity.render_mode)
    }

    #[inline]
    pub fn move_kind(&self) -> i32 {
        unsafe { *self.render_mode_address().byte_add(1).cast() }
    }

    /// only for base_players
    #[inline]
    unsafe fn is_dead_address(&self) -> *const u8 {
        self.networked(|networked| networked.base_player.is_dead)
    }

    /// only for base_players
    #[inline]
    pub fn view_angle(&self) -> &mut Vec3 {
        unsafe {
            let view_angle_address = self.is_dead_address().byte_add(4).as_mut().cast();

            &mut *view_angle_address
        }
    }

    /// only for base_players
    #[inline]
    pub fn velocity(&self) -> Vec3 {
        *self.networked(|networked| networked.base_player.velocity)
    }

    /// only for players
    #[inline]
    pub fn is_scoped(&self) -> bool {
        *self.networked(|networked| networked.player.is_scoped)
    }

    /// only for players
    /*#[inline]
    pub fn is_defusing(&self) -> bool {
        *self.networked(|networked| networked.player.is_defusing)
    }*/

    /// only for players
    #[inline]
    pub fn flags(&self) -> i32 {
        *self.networked(|networked| networked.player.flags)
    }

    /// only for players
    #[inline]
    pub fn armor(&self) -> i32 {
        *self.networked(|networked| networked.player.armor)
    }

    /// only for players
    #[inline]
    pub fn has_helmet(&self) -> bool {
        *self.networked(|networked| networked.player.has_helmet)
    }

    /// only for base players
    #[inline]
    pub fn view_offset(&self) -> Vec3 {
        *self.networked(|networked| networked.base_player.view_offset)
    }

    /// only for base players
    #[inline]
    pub fn eye_origin(&self) -> Vec3 {
        let origin = self.origin();
        let view_offset = self.view_offset();

        let z = if self.flags() & (1 << 1) != 0 {
            46.0
        } else {
            64.0
        };

        let view_offset = if view_offset == Vec3::zero() {
            Vec3::from_xyz(0.0, 0.0, z)
        } else {
            view_offset
        };

        origin + view_offset
    }

    /// only for fog
    #[inline]
    pub fn is_enabled(&self) -> &mut bool {
        self.networked(|networked| networked.fog.is_enabled)
    }

    /// only for fog
    #[inline]
    pub fn start_distance(&self) -> &mut f32 {
        self.networked(|networked| networked.fog.start)
    }

    /// only for fog
    #[inline]
    pub fn end_distance(&self) -> &mut f32 {
        self.networked(|networked| networked.fog.end)
    }

    /// only for fog
    #[inline]
    pub fn far_z(&self) -> &mut f32 {
        self.networked(|networked| networked.fog.far_z)
    }

    /// only for fog
    #[inline]
    pub fn density(&self) -> &mut f32 {
        self.networked(|networked| networked.fog.density)
    }

    /// only for fog
    #[inline]
    pub fn direction(&self) -> &mut Vec3 {
        self.networked(|networked| networked.fog.direction)
    }

    /// only for fog
    #[inline]
    pub fn color_primary(&self) -> &mut i32 {
        self.networked(|networked| networked.fog.color_primary)
    }

    /// only for fog
    #[inline]
    pub fn color_secondary(&self) -> &mut i32 {
        self.networked(|networked| networked.fog.color_secondary)
    }

    /// only for fog
    #[inline]
    pub fn hdr_scale(&self) -> &mut f32 {
        self.networked(|networked| networked.fog.hdr_scale)
    }

    /// only for tonemap
    #[inline]
    pub fn enable_min_exposure(&self) -> &mut bool {
        self.networked(|networked| networked.tonemap.enable_min_exposure)
    }

    /// only for tonemap
    #[inline]
    pub fn enable_max_exposure(&self) -> &mut bool {
        self.networked(|networked| networked.tonemap.enable_max_exposure)
    }

    /// only for tonemap
    #[inline]
    pub fn enable_bloom_scale(&self) -> &mut bool {
        self.networked(|networked| networked.tonemap.enable_bloom_scale)
    }

    /// only for tonemap
    #[inline]
    pub fn min_exposure(&self) -> &mut f32 {
        self.networked(|networked| networked.tonemap.min_exposure)
    }

    /// only for tonemap
    #[inline]
    pub fn max_exposure(&self) -> &mut f32 {
        self.networked(|networked| networked.tonemap.max_exposure)
    }

    /// only for tonemap
    #[inline]
    pub fn bloom_scale(&self) -> &mut f32 {
        self.networked(|networked| networked.tonemap.bloom_scale)
    }
}
