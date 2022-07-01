use super::{Contents, Displacement, HitGroup, Plane, Surface, SurfaceFlags};
use elysium_math::Vec3;

/// The summary of a trace.
#[repr(C)]
pub struct Summary {
    /// The origin position of the trace.
    #[doc(alias = "start")]
    #[doc(alias = "start_pos")]
    pub origin: Vec3,

    /// Either the hit position or end of the trace.
    #[doc(alias = "end")]
    #[doc(alias = "end_pos")]
    pub hit_pos: Vec3,

    /// Contains information about the plane the trace hit.
    /// NOTE: plane can be invalid! If the trace never leaves a solid, there will be no valid
    /// plane.
    plane: Plane,

    pub fraction: f32,

    /// The contents of the hit surface.
    pub contents: Contents,

    /// The displacement flags of the hit surface.
    #[doc(alias = "disp_flags")]
    pub displacement: Displacement,

    /// Is the entire trace is within a solid?
    ///
    /// If this is `true`, `plane` is `None`.
    #[doc(alias = "all_solid")]
    pub within_solid: bool,

    /// Indicates whether the trace started within a solid.
    #[doc(alias = "start_solid")]
    pub start_within_solid: bool,

    /// Given the trace started within a solid, return the fraction of which the trace left the
    /// solid.
    #[doc(alias = "fraction_left_solid")]
    pub fraction_exited_solid: f32,

    /// The surface this trace hits.
    pub surface: Surface,

    /// The hitgroup hit.
    pub hit_group: HitGroup,

    pub physics_bone: i32,

    pub world_surface_index: u16,

    /// The entity hit (if present).
    pub entity: *const u8,

    /// The hitbox hit.
    pub hitbox: i32,
}

impl Summary {
    /// Returns information about the plane the trace hit.
    ///
    /// Plane is `None` if the trace never left a solid.
    pub fn plane(&self) -> Option<&Plane> {
        self.within_solid.then(|| &self.plane)
    }

    /// Returns information about the plane the trace hit.
    ///
    /// # Safety
    ///
    /// If `within_solid` is `true`, this is invalid.
    pub unsafe fn plane_unchecked(&self) -> &Plane {
        &self.plane
    }

    /// Returns `true` if it hit something.
    pub fn did_hit(&self) -> bool {
        !self.entity.is_null()
    }

    /// convenience method for `surface.has_flag()`
    pub fn has_flag(&self, flag: SurfaceFlags) -> bool {
        self.surface.has_flag(flag)
    }
}
