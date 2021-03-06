use crate::Networked;
use elysium_math::Vec3;
use elysium_menu::Menu;
use elysium_sdk::{Globals, Input, Interfaces, Vars};
use iced_glow::glow;
use iced_native::{Point, Size};
use std::cell::SyncUnsafeCell;
use std::ptr;

pub use cache::{Player, Players};
pub use hooks::*;
pub use local::Local;
pub use materials::Materials;

mod cache;
mod hooks;
mod local;
mod materials;

#[repr(transparent)]
struct Wrap(State);

unsafe impl Sync for Wrap {}

static SHARED: SyncUnsafeCell<Wrap> = SyncUnsafeCell::new(Wrap(NEW));

const NEW: State = State {
    context: None,
    get_proc_address: None,
    menu: None,
    menu_open: (false, false),
    cursor_position: Point::new(0.0, 0.0),
    window_size: Size::new(0, 0),
    hooks: None,
    networked: Networked::new(),
    vars: None,
    interfaces: None,
    globals: None,
    input: None,
    players: Players::new(),
    local: Local::new(),
    materials: Materials::new(),
    send_packet: ptr::null_mut(),
    view_angle: Vec3::zero(),
};

/// variables that need to be shared between hooks
pub struct State {
    /// opengl context
    pub context: Option<glow::Context>,
    /// opengl get proc address
    pub get_proc_address: Option<unsafe extern "C" fn(symbol: *const u8) -> *const u8>,
    /// menu context
    pub menu: Option<Menu>,
    /// first boolean determines whether the menu is visible, second prevents input from being
    /// spaz
    pub menu_open: (bool, bool),
    /// the cursor position
    pub cursor_position: Point,
    /// csgos window size
    pub window_size: Size<u32>,
    /// csgo, sdl, etc hooks
    pub hooks: Option<Hooks>,
    /// netvars
    pub networked: Networked,
    /// cvars
    pub vars: Option<Vars>,
    /// source engine interfaces
    pub interfaces: Option<Interfaces>,
    /// globals
    pub globals: Option<&'static mut Globals>,
    /// cinput
    pub input: Option<&'static mut Input>,
    /// efficient cache of players and their data (btw why is entitylist a linked list?)
    pub players: Players,
    pub materials: Materials,
    /// local player variables
    pub local: Local,
    /// cl_move send_packet
    pub send_packet: *mut bool,
    /// engine view_angle
    pub view_angle: Vec3,
}

impl State {
    #[inline]
    pub fn get() -> &'static mut State {
        // SAFETY: Wrap is repr(transparent)
        unsafe { &mut *SyncUnsafeCell::raw_get(&SHARED).cast() }
    }

    /// toggle menu
    #[inline]
    pub fn toggle_menu(&mut self) {
        if !self.menu_open.1 {
            self.menu_open.0 ^= true;
            self.menu_open.1 = true;
        }
    }

    /// release menu toggle lock
    #[inline]
    pub fn release_menu_toggle(&mut self) {
        self.menu_open.1 = false;
    }
}
