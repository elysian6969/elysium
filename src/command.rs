use sdk::{Angle, Vector};

pub const IN_ATTACK: i32 = 1 << 0;
pub const IN_JUMP: i32 = 1 << 1;
pub const IN_DUCK: i32 = 1 << 2;
pub const IN_BULLRUSH: i32 = 1 << 22;

#[derive(Debug)]
#[repr(C)]
pub struct Command {
    vtable: *const (),
    pub command_number: i32,
    pub tick_count: i32,
    pub view_angle: Angle,
    pub aim_direction: Angle,
    pub forward_move: f32,
    pub side_move: f32,
    pub up_move: f32,
    pub state: i32,
    pub impulse: u8,
    pub weapon_select: i32,
    pub weapon_subtype: i32,
    pub random_seed: i32,
    pub mouse_dx: i16,
    pub mouse_dy: i16,
    pub has_been_predicted: bool,
    pub head_angles: Angle,
    pub head_offset: Vector,
}

impl Command {
    const fn has(&self, flag: i32) -> bool {
        (self.state & flag) != 0
    }

    pub const fn set(&mut self, flag: i32, value: bool) {
        if value {
            self.state |= flag;
        } else {
            self.state &= !flag;
        }
    }

    pub const fn in_attack(&self) -> bool {
        self.has(IN_ATTACK)
    }

    pub const fn in_jump(&self) -> bool {
        self.has(IN_JUMP)
    }

    pub const fn in_duck(&self) -> bool {
        self.has(IN_DUCK)
    }

    pub const fn in_fast_duck(&self) -> bool {
        self.has(IN_DUCK | IN_BULLRUSH)
    }

    pub const fn attack(&mut self, value: bool) {
        self.set(IN_ATTACK, value)
    }

    pub const fn jump(&mut self, value: bool) {
        self.set(IN_JUMP, value)
    }

    pub const fn duck(&mut self, value: bool) {
        self.set(IN_DUCK, value)
    }

    pub const fn fast_duck(&mut self, value: bool) {
        self.set(IN_DUCK | IN_BULLRUSH, value);
    }
}
