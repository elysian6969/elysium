use crate::client::Client;
use crate::command::Command;
use crate::console::{Console, Var};
use crate::engine::Engine;
use crate::entity::{Entity, EntityList};
use crate::frame::Frame;
use crate::globals::Globals;
use crate::hooks;
use crate::input::Input;
use crate::interfaces::Interfaces;
use crate::libraries::Libraries;
use crate::material::Material;
use crate::model::{DrawModelState, ModelInfo, ModelRender, ModelRenderInfo};
use crate::movement::Movement;
use crate::netvars;
use crate::physics::Physics;
use crate::trace::Tracer;
use crate::Result;
use core::ptr;
use sdk::Matrix3x4;
use std::lazy::SyncOnceCell;
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use std::sync::Arc;
use vptr::VirtualMut;

pub type OnFrame = Box<dyn Fn(Frame) + 'static>;
pub type OnMove = Box<dyn Fn(Movement) -> Movement + 'static>;

static GLOBAL: SyncOnceCell<Global> = SyncOnceCell::new();

pub(crate) struct GlobalRef {
    libraries: Libraries,
    interfaces: Interfaces,
    on_frame: OnFrame,
    on_move: OnMove,
    tick: AtomicU32,
    last_command_has_been_predicted: AtomicBool,
    create_move_original: Box<hooks::create_move::Signature>,
    frame_stage_notify_original: Box<hooks::frame_stage_notify::Signature>,
    draw_model_execute_original: Box<hooks::draw_model_execute::Signature>,
    local_player: Box<Option<Entity>>,
}

#[derive(Clone)]
pub struct Global(pub(crate) Arc<GlobalRef>);

unsafe impl Send for GlobalRef {}
unsafe impl Sync for GlobalRef {}

impl Global {
    pub fn init() -> Result<Self> {
        println!("init libs");

        let libraries = Libraries::new()?;

        println!("init interfaces");

        let interfaces = Interfaces::new(&libraries);

        println!("{:?}", &interfaces);

        let this = Self(Arc::new(GlobalRef {
            libraries,
            interfaces,
            on_frame: Box::new(move |_frame| {}),
            on_move: Box::new(move |movement| movement),
            tick: AtomicU32::new(0),
            last_command_has_been_predicted: AtomicBool::new(false),
            // TODO: replace these with dummies
            create_move_original: Box::new(hooks::create_move::hook),
            frame_stage_notify_original: Box::new(hooks::frame_stage_notify::hook),
            draw_model_execute_original: Box::new(hooks::draw_model_execute::hook),
            local_player: Box::new(None),
        }));

        println!("created global");

        let _ = GLOBAL.set(this.clone());

        println!("set global");

        unsafe {
            ptr::write(
                this.create_move_original_ptr(),
                this.interfaces()
                    .client_mode
                    .vreplace_protected(hooks::create_move::hook, 25 * 8),
            );

            println!("hooked create_move");

            ptr::write(
                this.frame_stage_notify_original_ptr(),
                (this.interfaces().client.as_ptr() as *mut ())
                    .vreplace_protected(hooks::frame_stage_notify::hook, 37 * 8),
            );

            println!("hooked frame_stage_notify");

            ptr::write(
                this.draw_model_execute_original_ptr(),
                (this.interfaces().model_render.as_ptr() as *mut ())
                    .vreplace_protected(hooks::draw_model_execute::hook, 21 * 8),
            );

            println!("hooked draw_model_execute");
        }

        netvars::set(this.client());

        Ok(this)
    }

    pub fn handle() -> &'static Global {
        unsafe { GLOBAL.get().unwrap_unchecked() }
    }

    /// Current client time.
    pub fn client_time(&self) -> f32 {
        self.0.interfaces.globals.current_time
    }

    /// The interval (in seconds) that one tick takes.
    ///
    /// 1 second / 64 ticks = 0.015625 seconds
    /// 1 second / 128 ticks = 0.0078125 seconds
    pub fn interval_per_tick(&self) -> f32 {
        self.0.interfaces.globals.interval_per_tick
    }

    pub fn tick(&self) -> u32 {
        self.0.tick.load(Ordering::SeqCst)
    }

    pub fn set_tick(&self, tick: u32) {
        self.0.tick.store(tick, Ordering::SeqCst);
    }

    pub fn increment_tick(&self) {
        self.0.tick.fetch_add(1, Ordering::SeqCst);
    }

    pub fn last_command_has_been_predicted(&self) -> bool {
        self.0
            .last_command_has_been_predicted
            .load(Ordering::SeqCst)
    }

    pub fn set_last_command_has_been_predicted(&self, predicted: bool) {
        self.0
            .last_command_has_been_predicted
            .store(predicted, Ordering::SeqCst);
    }

    pub fn libraries(&self) -> &Libraries {
        &self.0.libraries
    }

    pub fn interfaces(&self) -> &Interfaces {
        &self.0.interfaces
    }

    pub fn globals(&self) -> &Globals {
        self.0.interfaces.globals
    }

    pub fn physics(&self) -> &Physics {
        &self.0.interfaces.physics
    }

    pub fn input(&self) -> &Input {
        self.0.interfaces.input
    }

    pub fn engine(&self) -> &Engine {
        &self.0.interfaces.engine
    }

    pub fn entity_list(&self) -> &EntityList {
        &self.0.interfaces.entity_list
    }

    pub fn tracer(&self) -> &Tracer {
        &self.0.interfaces.tracer
    }

    pub fn client(&self) -> &Client {
        &self.0.interfaces.client
    }

    pub fn model_render(&self) -> &ModelRender {
        &self.0.interfaces.model_render
    }

    pub fn model_info(&self) -> &ModelInfo {
        &self.0.interfaces.model_info
    }

    pub fn console(&self) -> &Console {
        &self.0.interfaces.console
    }

    pub fn animation_layers(&self) -> u32 {
        self.0.interfaces.animation_layers
    }

    pub fn animation_state(&self) -> u32 {
        self.0.interfaces.animation_state
    }

    pub fn cheats(&self) -> &Var<i32> {
        &self.0.interfaces.cheats
    }

    pub fn ffa(&self) -> &Var<i32> {
        &self.0.interfaces.ffa
    }

    pub fn gravity(&self) -> &Var<f32> {
        &self.0.interfaces.gravity
    }

    pub fn infinite_ammo(&self) -> &Var<i32> {
        &self.0.interfaces.infinite_ammo
    }

    pub fn lost_focus_sleep(&self) -> &Var<i32> {
        &self.0.interfaces.lost_focus_sleep
    }

    pub fn model_stats_overlay(&self) -> &Var<i32> {
        &self.0.interfaces.model_stats_overlay
    }

    pub fn panorama_blur(&self) -> &Var<i32> {
        &self.0.interfaces.panorama_blur
    }

    pub fn physics_timescale(&self) -> &Var<f32> {
        &self.0.interfaces.physics_timescale
    }

    pub fn post_processing(&self) -> &Var<i32> {
        &self.0.interfaces.post_processing
    }

    pub fn ragdoll_gravity(&self) -> &Var<f32> {
        &self.0.interfaces.ragdoll_gravity
    }

    pub fn show_impacts(&self) -> &Var<i32> {
        &self.0.interfaces.show_impacts
    }

    pub(crate) fn on_frame_ptr(&self) -> *mut OnFrame {
        &self.0.on_frame as *const OnFrame as *mut OnFrame
    }

    pub(crate) fn on_move_ptr(&self) -> *mut OnMove {
        &self.0.on_move as *const OnMove as *mut OnMove
    }

    pub(crate) fn create_move_original_ptr(&self) -> *mut hooks::create_move::Signature {
        &*self.0.create_move_original as *const hooks::create_move::Signature
            as *mut hooks::create_move::Signature
    }

    pub(crate) fn create_move_original(
        &self,
        this: *const (),
        input_sample_time: f32,
        command: &mut Command,
    ) -> bool {
        let original = unsafe { *self.create_move_original_ptr() };

        unsafe { original(this, input_sample_time, command) }
    }

    pub(crate) fn frame_stage_notify_original_ptr(
        &self,
    ) -> *mut hooks::frame_stage_notify::Signature {
        &*self.0.frame_stage_notify_original as *const hooks::frame_stage_notify::Signature
            as *mut hooks::frame_stage_notify::Signature
    }

    pub(crate) fn frame_stage_notify_original(&self, this: *const (), frame: Frame) {
        let original = unsafe { *self.frame_stage_notify_original_ptr() };

        unsafe { original(this, frame) }
    }

    pub(crate) fn draw_model_execute_original_ptr(
        &self,
    ) -> *mut hooks::draw_model_execute::Signature {
        &*self.0.draw_model_execute_original as *const hooks::draw_model_execute::Signature
            as *mut hooks::draw_model_execute::Signature
    }

    pub(crate) fn draw_model_execute_original(
        &self,
        this: *const (),
        context: *const (),
        state: *const DrawModelState,
        info: *const ModelRenderInfo,
        bone_to_world: *const Matrix3x4,
    ) {
        let original = unsafe { *self.draw_model_execute_original_ptr() };

        unsafe { original(this, context, state, info, bone_to_world) }
    }

    pub(crate) fn local_player_ptr(&self) -> *mut Box<Option<Entity>> {
        &self.0.local_player as *const Box<Option<Entity>> as *mut Box<Option<Entity>>
    }

    pub(crate) fn local_player(&self) -> Option<&Entity> {
        (*self.0.local_player).as_ref()
    }

    /// set frame stage notify hook
    pub fn on_frame<F>(&self, f: F)
    where
        F: Fn(Frame) + 'static,
    {
        unsafe {
            ptr::write(self.on_frame_ptr(), Box::new(f));
        }
    }

    /// set create move hook
    pub fn on_move<F>(&self, f: F)
    where
        F: Fn(Movement) -> Movement + 'static,
    {
        unsafe {
            ptr::write(self.on_move_ptr(), Box::new(f));
        }
    }

    pub fn flat_material(&self) -> &Material {
        &self.0.interfaces.flat
    }
}
