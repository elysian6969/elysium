use super::Contents;
use core::ops;

/// Mask for a trace.
#[repr(C)]
pub struct Mask(pub u32);

impl Mask {
    pub const ALL: Self = Self(0xFFFFFFFF);

    /// everything that is normally solid
    pub const SOLID: Self = Self(
        Contents::SOLID.0
            | Contents::MOVEABLE.0
            | Contents::WINDOW.0
            | Contents::MONSTER.0
            | Contents::GRATE.0,
    );

    /// everything that blocks player movement
    pub const PLAYERSOLID: Self = Self(
        Contents::SOLID.0
            | Contents::MOVEABLE.0
            | Contents::PLAYERCLIP.0
            | Contents::WINDOW.0
            | Contents::MONSTER.0
            | Contents::GRATE.0,
    );

    /// blocks npc movement
    pub const NPCSOLID: Self = Self(
        Contents::SOLID.0
            | Contents::MOVEABLE.0
            | Contents::MONSTERCLIP.0
            | Contents::WINDOW.0
            | Contents::MONSTER.0
            | Contents::GRATE.0,
    );

    /// water physics in these contents
    pub const WATER: Self = Self(Contents::WATER.0 | Contents::MOVEABLE.0 | Contents::SLIME.0);

    /// everything that blocks line of sight for ai, lighting, etc
    pub const OPAQUE: Self = Self(Contents::SOLID.0 | Contents::MOVEABLE.0 | Contents::OPAQUE.0);

    /// everything that blocks line of sight for ai, lighting, etc, but with monsters added
    pub const OPAQUE_AND_NPCS: Self = Self(Self::OPAQUE.0 | Contents::MONSTER.0);

    /// everything that blocks line of sight for players
    pub const VISIBLE: Self = Self(Self::OPAQUE.0 | Contents::IGNORE_NODRAW_OPAQUE.0);

    /// everything that blocks line of sight for players, but with monsters added
    pub const VISIBLE_AND_NPCS: Self =
        Self(Self::OPAQUE_AND_NPCS.0 | Contents::IGNORE_NODRAW_OPAQUE.0);

    /// bullets see these as solid
    pub const SHOT: Self = Self(
        Contents::SOLID.0
            | Contents::MOVEABLE.0
            | Contents::MONSTER.0
            | Contents::WINDOW.0
            | Contents::DEBRIS.0
            | Contents::HITBOX.0,
    );

    /// non-raycasted weapons see this as solid (includes grates)
    pub const SHOT_HULL: Self = Self(
        Contents::SOLID.0
            | Contents::MOVEABLE.0
            | Contents::MONSTER.0
            | Contents::WINDOW.0
            | Contents::DEBRIS.0
            | Contents::GRATE.0,
    );

    /// hits solids (not grates) and passes through everything else
    pub const SHOT_PORTAL: Self =
        Self(Contents::SOLID.0 | Contents::MOVEABLE.0 | Contents::WINDOW.0);

    /// everything normally solid, except monsters (world+brush only)
    pub const SOLID_BRUSHONLY: Self =
        Self(Contents::SOLID.0 | Contents::MOVEABLE.0 | Contents::WINDOW.0 | Contents::GRATE.0);

    /// everything normally solid for player movement, except monsters (world+brush only)
    pub const PLAYERSOLID_BRUSHONLY: Self = Self(
        Contents::SOLID.0
            | Contents::MOVEABLE.0
            | Contents::WINDOW.0
            | Contents::PLAYERCLIP.0
            | Contents::GRATE.0,
    );

    /// everything normally solid for npc movement, except monsters (world+brush only)
    pub const NPCSOLID_BRUSHONLY: Self = Self(
        Contents::SOLID.0
            | Contents::MOVEABLE.0
            | Contents::WINDOW.0
            | Contents::MONSTERCLIP.0
            | Contents::GRATE.0,
    );

    /// just the world, used for route rebuilding
    pub const NPCWORLDSTATIC: Self =
        Self(Contents::SOLID.0 | Contents::WINDOW.0 | Contents::MONSTERCLIP.0 | Contents::GRATE.0);

    /// These are things that can split areaportals
    pub const SPLITAREAPORTAL: Self = Self(Contents::WATER.0 | Contents::SLIME.0);
}

impl const ops::BitOr<Contents> for Mask {
    type Output = u32;

    fn bitor(self, rhs: Contents) -> u32 {
        self.0 | rhs.0
    }
}
