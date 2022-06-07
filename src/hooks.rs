//! Function hooks.

use crate::{state, Entity};
use core::mem;
use core::mem::MaybeUninit;
use core::ptr::NonNull;
use elysium_math::Vec3;
use elysium_sdk::{Command, Engine, EntityList, Frame, Globals, Input};
use iced_elysium_gl::Viewport;
use iced_native::Size;

/// `SDL_GL_SwapWindow` hook.
#[inline(never)]
pub unsafe extern "C" fn swap_window(sdl_window: *mut sdl2_sys::SDL_Window) {
    let mut width = MaybeUninit::uninit();
    let mut height = MaybeUninit::uninit();

    sdl2_sys::SDL_GetWindowSize(sdl_window, width.as_mut_ptr(), height.as_mut_ptr());

    let width = width.assume_init();
    let height = height.assume_init();
    let size = Size::new(width as u32, height as u32);

    state::update_window_size(size);

    let context = state::gl_context();

    // enable auto-conversion from/to sRGB
    context.enable(elysium_gl::FRAMEBUFFER_SRGB);

    // enable alpha blending to not break our fonts
    context.enable(elysium_gl::BLEND);
    context.blend_func(elysium_gl::SRC_ALPHA, elysium_gl::ONE_MINUS_SRC_ALPHA);

    let viewport = Viewport::with_physical_size(size, 1.0);
    let menu = state::menu(context, viewport.clone());

    //if state::is_menu_open() {
    context.viewport(0, 0, size.width as i32, size.height as i32);

    menu.update(viewport.clone(), state::cursor_position());
    menu.draw(context, viewport);
    //}

    // disable auto-conversion from/to sRGB
    context.enable(elysium_gl::FRAMEBUFFER_SRGB);

    // disable alpha blending to not break vgui fonts
    context.disable(elysium_gl::BLEND);

    state::hooks::swap_window(sdl_window);
}

/// `SDL_PollEvent` hook.
#[inline(never)]
pub unsafe extern "C" fn poll_event(sdl_event: *mut sdl2_sys::SDL_Event) -> i32 {
    let result = state::hooks::poll_event(sdl_event);

    if !state::is_menu_none() {
        let menu = state::menu_unchecked();

        elysium_input::map_event(*sdl_event, |event| {
            use iced_native::{keyboard, mouse, Event};

            match &event {
                Event::Keyboard(keyboard::Event::KeyPressed {
                    key_code: keyboard::KeyCode::Insert,
                    ..
                }) => state::toggle_menu(),
                Event::Mouse(mouse::Event::CursorMoved { position }) => {
                    state::update_cursor_position(*position)
                }
                _ => {}
            };

            menu.queue_event(event)
        });
    }

    // block input to the game when the menu is open
    if state::is_menu_open() {
        (*sdl_event).type_ = 0;
    }

    result
}

/// `CL_Move` hook.
#[inline(never)]
pub unsafe extern "C" fn cl_move(_accumulated_extra_samples: f32, _final_tick: bool) {
    return;
}

#[inline(never)]
pub unsafe extern "C" fn frame_stage_notify(this: *const (), frame: i32) {
    let engine = &*state::engine().cast::<Engine>();
    let entity_list = &*state::entity_list().cast::<EntityList>();
    let globals = &*state::globals().cast::<Globals>();
    let input = &*state::input().cast::<Input>();

    let frame: Frame = mem::transmute(frame);
    let index = engine.local_player_index();
    let entity = entity_list.get(index);

    if entity.is_null() {
        state::local::set_aim_punch_angle(Vec3::zero());
        state::local::set_view_punch_angle(Vec3::zero());
        state::local::set_player_none();
    } else {
        state::local::set_player(NonNull::new_unchecked(entity.as_mut()));

        let entity = &*entity.cast::<Entity>();

        match frame {
            Frame::RenderStart => {
                if input.thirdperson {
                    // fix the local player's view_angle when in thirdperson
                    *entity.view_angle() = state::local::view_angle();
                } else {
                    // in coordinance with override_view, this will change the view model's position.

                    if state::local::use_shot_view_angle() != 0.0 {
                        if state::local::use_shot_view_angle() > globals.current_time {
                            *entity.view_angle() = state::local::shot_view_angle();
                        } else {
                            *entity.view_angle() = *state::view_angle();
                            state::local::set_use_shot_view_angle(0.0);
                        }
                    }

                    // rotate view model
                    entity.view_angle().z = 15.0;
                }
            }
            _ => {
                if input.thirdperson {
                    // restore to the expected value
                    *entity.view_angle() = *state::view_angle();
                }
            }
        }
    }

    state::hooks::frame_stage_notify(this, frame as i32);
}

#[inline(never)]
pub unsafe extern "C" fn write_user_command_delta_to_buffer(
    _this: *const u8,
    slot: i32,
    buffer: *mut u8,
    from: i32,
    to: i32,
    _new_command: u8,
) -> bool {
    let mut zero_command = MaybeUninit::<Command>::zeroed();
    let zero_command = zero_command.as_mut_ptr();
    let input = &*state::input().cast::<Input>();

    let from_command = if from == -1 {
        zero_command
    } else {
        let from_command = input.get_user_command(slot, from).as_mut();

        if from_command.is_null() {
            zero_command
        } else {
            from_command
        }
    };

    let to_command = input.get_user_command(slot, to).as_mut();
    let to_command = if to_command.is_null() {
        zero_command
    } else {
        to_command
    };

    let from_command = from_command.cast();
    let to_command = to_command.as_const().cast();

    state::hooks::write_user_command(buffer, to_command, from_command);

    true
}
