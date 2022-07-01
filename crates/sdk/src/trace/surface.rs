use super::SurfaceFlags;

/// The surface hit by a trace.
#[repr(C)]
pub struct Surface {
    /// The name of the surface.
    pub name: *const u8,

    /// An integer used in finding the material of the surface hit.
    pub properties: i16,

    /// Flags of the surface.
    ///
    /// Used to filter unwanted surfaces.
    pub flags: SurfaceFlags,
}

impl Surface {
    pub fn has_flag(&self, flag: SurfaceFlags) -> bool {
        (self.flags & flag) != SurfaceFlags::EMPTY
    }
}
