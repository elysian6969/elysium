use core::ops;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(transparent)]
pub struct State(pub i32);

impl State {
    pub const ATTACK: Self = Self::new(1 << 0);
    pub const ATTACK2: Self = Self::new(1 << 11);
    pub const ATTACK3: Self = Self::new(1 << 25);
    pub const ANY_ATTACK: Self = Self::ATTACK | Self::ATTACK2 | Self::ATTACK3;

    pub const JUMP: Self = Self::new(1 << 1);
    pub const CROUCH: Self = Self::new(1 << 2);
    pub const FORWARD: Self = Self::new(1 << 3);
    pub const BACKWARD: Self = Self::new(1 << 4);
    pub const USE: Self = Self::new(1 << 5);
    pub const CANCEL: Self = Self::new(1 << 6);
    pub const LEFT: Self = Self::new(1 << 7);
    pub const RIGHT: Self = Self::new(1 << 8);
    pub const MOVE_LEFT: Self = Self::new(1 << 9);
    pub const MOVE_RIGHT: Self = Self::new(1 << 10);
    pub const RUN: Self = Self::new(1 << 12);
    pub const RELOAD: Self = Self::new(1 << 13);
    pub const ALT1: Self = Self::new(1 << 14);
    pub const ALT2: Self = Self::new(1 << 15);
    pub const SCORE: Self = Self::new(1 << 16);
    pub const SPEED: Self = Self::new(1 << 17);
    pub const WALK: Self = Self::new(1 << 18);
    pub const ZOOM: Self = Self::new(1 << 19);
    pub const WEAPON1: Self = Self::new(1 << 20);
    pub const WEAPON2: Self = Self::new(1 << 21);
    pub const BULLRUSH: Self = Self::new(1 << 22);
    pub const GRENADE1: Self = Self::new(1 << 23);
    pub const GRENADE2: Self = Self::new(1 << 24);

    const fn new(state: i32) -> Self {
        Self(state)
    }

    pub const fn is_empty(&self) -> bool {
        self.0 == 0
    }

    pub const fn in_attack(&self) -> bool {
        !(*self & Self::ATTACK).is_empty()
    }

    pub const fn in_attack2(&self) -> bool {
        !(*self & Self::ATTACK2).is_empty()
    }

    pub const fn in_attack3(&self) -> bool {
        !(*self & Self::ATTACK2).is_empty()
    }

    pub const fn in_any_attack(&self) -> bool {
        !(*self & Self::ANY_ATTACK).is_empty()
    }

    pub const fn in_jump(&self) -> bool {
        self.0 & Self::JUMP.0 != 0
    }

    pub const fn in_crouch(&self) -> bool {
        self.0 & Self::CROUCH.0 != 0
    }

    pub const fn in_forward(&self) -> bool {
        self.0 & Self::FORWARD.0 != 0
    }

    pub const fn in_backward(&self) -> bool {
        self.0 & Self::BACKWARD.0 != 0
    }

    pub const fn in_use(&self) -> bool {
        self.0 & Self::USE.0 != 0
    }

    pub const fn in_cancel(&self) -> bool {
        self.0 & Self::CANCEL.0 != 0
    }

    pub const fn in_left(&self) -> bool {
        self.0 & Self::LEFT.0 != 0
    }

    pub const fn in_right(&self) -> bool {
        self.0 & Self::RIGHT.0 != 0
    }

    pub const fn in_move_right(&self) -> bool {
        self.0 & Self::MOVE_RIGHT.0 != 0
    }

    pub const fn in_move_left(&self) -> bool {
        self.0 & Self::MOVE_LEFT.0 != 0
    }

    pub const fn in_run(&self) -> bool {
        self.0 & Self::RUN.0 != 0
    }

    pub const fn in_reload(&self) -> bool {
        self.0 & Self::RELOAD.0 != 0
    }

    pub const fn in_alt1(&self) -> bool {
        self.0 & Self::ALT1.0 != 0
    }

    pub const fn in_alt2(&self) -> bool {
        self.0 & Self::ALT2.0 != 0
    }

    pub const fn in_score(&self) -> bool {
        self.0 & Self::SCORE.0 != 0
    }

    pub const fn in_speed(&self) -> bool {
        self.0 & Self::SPEED.0 != 0
    }

    pub const fn in_walk(&self) -> bool {
        self.0 & Self::WALK.0 != 0
    }

    pub const fn in_zoom(&self) -> bool {
        self.0 & Self::ZOOM.0 != 0
    }

    pub const fn in_weapon1(&self) -> bool {
        self.0 & Self::WEAPON1.0 != 0
    }

    pub const fn in_weapon2(&self) -> bool {
        self.0 & Self::WEAPON2.0 != 0
    }

    pub const fn in_bullrush(&self) -> bool {
        self.0 & Self::BULLRUSH.0 != 0
    }

    pub const fn in_grenade1(&self) -> bool {
        (*self & Self::GRENADE1).0 != 0
    }

    pub const fn in_grenade2(&self) -> bool {
        (*self & Self::GRENADE2).0 != 0
    }
}

impl const ops::BitAnd for State {
    type Output = State;

    fn bitand(self, rhs: State) -> State {
        State(self.0 & rhs.0)
    }
}

impl const ops::BitAndAssign for State {
    fn bitand_assign(&mut self, rhs: State) {
        self.0 &= rhs.0;
    }
}

impl const ops::BitOr for State {
    type Output = State;

    fn bitor(self, rhs: State) -> State {
        State(self.0 | rhs.0)
    }
}

impl const ops::BitOrAssign for State {
    fn bitor_assign(&mut self, rhs: State) {
        self.0 |= rhs.0;
    }
}

impl const ops::BitXor for State {
    type Output = State;

    fn bitxor(self, rhs: State) -> State {
        State(self.0 ^ rhs.0)
    }
}

impl const ops::BitXorAssign for State {
    fn bitxor_assign(&mut self, rhs: State) {
        self.0 ^= rhs.0;
    }
}

impl const ops::Not for State {
    type Output = State;

    fn not(self) -> State {
        Self(!self.0)
    }
}
