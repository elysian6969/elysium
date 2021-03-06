/// The hit group a trace hits.
#[derive(Debug)]
#[repr(i32)]
pub enum HitGroup {
    Generic = 0,
    Head = 1,
    Chest = 2,
    Stomach = 3,
    LeftArm = 4,
    RightArm = 5,
    LeftLeg = 6,
    RightLeg = 7,
    Gear = 10,
}

impl HitGroup {
    /// Damage multipler for this hit group.
    pub const fn damage_multiplier(&self) -> f32 {
        use HitGroup::{Head, LeftLeg, RightLeg, Stomach};

        match self {
            Head => 4.0,
            Stomach => 1.25,
            LeftLeg | RightLeg => 0.75,
            _ => 1.0,
        }
    }

    /// Actually hits?
    pub const fn is_hit(&self) -> bool {
        use HitGroup::*;

        matches!(
            *self,
            Head | Chest | Stomach | LeftArm | RightArm | LeftLeg | RightLeg
        )
    }
}
