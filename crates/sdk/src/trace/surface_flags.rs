use core::{cmp, ops};

/// Surface flags.
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct SurfaceFlags(u16);

impl SurfaceFlags {
    pub const EMPTY: Self = Self(0);

    /// value will hold the light strength
    pub const LIGHT: Self = Self(0x0001);

    /// don't draw, indicates we should skylight + draw 2d sky but not draw the 3d skybox
    pub const SKY2D: Self = Self(0x0002);

    /// don't draw, but add to skybox
    pub const SKY: Self = Self(0x0004);

    /// turbulent water warp
    pub const WARP: Self = Self(0x0008);

    pub const TRANS: Self = Self(0x0010);

    /// the surface can not have a portal placed on it
    pub const NO_PORTAL: Self = Self(0x0020);

    /// FIXME: this is an xbox hack to work around elimination of trigger surfaces, which breaks occluders
    pub const TRIGGER: Self = Self(0x0040);

    /// don't bother referencing the texture
    pub const NO_DRAW: Self = Self(0x0080);

    /// make a primary bsp splitter
    pub const HINT: Self = Self(0x0100);

    /// completely ignore, allowing non-closed brushes
    pub const SKIP: Self = Self(0x0200);

    /// don't calculate light
    pub const NO_LIGHT: Self = Self(0x0400);

    /// calculate three lightmaps for the surface for bumpmapping
    pub const BUMPLIGHT: Self = Self(0x0800);

    /// don't receive shadows
    pub const NO_SHADOWS: Self = Self(0x1000);

    /// don't receive decals
    pub const NO_DECALS: Self = Self(0x2000);

    /// the surface can not have paint placed on it
    pub const NO_PAINT: Self = Self(0x2000);

    /// don't subdivide patches on this surface
    pub const NO_CHOP: Self = Self(0x4000);

    /// surface is part of a hitbox
    pub const HITBOX: Self = Self(0x8000);
}

impl const ops::BitAnd for SurfaceFlags {
    type Output = SurfaceFlags;

    fn bitand(self, rhs: SurfaceFlags) -> SurfaceFlags {
        Self(self.0 & rhs.0)
    }
}

impl const ops::BitAndAssign for SurfaceFlags {
    fn bitand_assign(&mut self, rhs: SurfaceFlags) {
        self.0 &= rhs.0
    }
}

impl const ops::BitOr for SurfaceFlags {
    type Output = SurfaceFlags;

    fn bitor(self, rhs: SurfaceFlags) -> SurfaceFlags {
        Self(self.0 | rhs.0)
    }
}

impl const ops::BitOrAssign for SurfaceFlags {
    fn bitor_assign(&mut self, rhs: SurfaceFlags) {
        self.0 |= rhs.0
    }
}

impl const ops::Not for SurfaceFlags {
    type Output = SurfaceFlags;

    fn not(self) -> SurfaceFlags {
        Self(!self.0)
    }
}

impl const cmp::PartialEq for SurfaceFlags {
    fn eq(&self, rhs: &SurfaceFlags) -> bool {
        self.0 == rhs.0
    }

    fn ne(&self, rhs: &SurfaceFlags) -> bool {
        self.0 != rhs.0
    }
}
