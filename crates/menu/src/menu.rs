//! Menu related functions.

use crate::{assets, Controls};
use iced_glow::{glow, Backend, Renderer, Settings, Viewport};
use iced_native::clipboard::Null;
use iced_native::program::State;
use iced_native::{clipboard, renderer, Color, Debug, Event, Point};

/// Menu state and rendering structures.
pub struct Menu {
    clipboard: Null,
    debug: Debug,
    renderer: Renderer,
    //scene: Scene,
    state: State<Controls>,
}

impl Menu {
    #[inline]
    pub fn new(context: &glow::Context, viewport: Viewport) -> Self {
        let clipboard = clipboard::Null;
        let controls = Controls::new();
        let mut debug = Debug::new();
        let mut renderer = Renderer::new(Backend::new(
            context,
            Settings {
                default_font: Some(assets::QUICKSAND_REGULAR),
                ..Settings::default()
            },
        ));

        let state = State::new(controls, viewport.logical_size(), &mut renderer, &mut debug);
        let debug = debug;
        let renderer = renderer;

        Self {
            clipboard,
            debug,
            renderer,
            //scene,
            state,
        }
    }

    #[inline]
    pub fn draw(&mut self, context: &glow::Context, viewport: Viewport) {
        let debug = &mut self.debug;
        let renderer = &mut self.renderer;

        renderer.with_primitives(|backend, primitives| {
            backend.present(&context, primitives, &viewport, &debug.overlay());
        });
    }

    #[inline]
    pub fn update(&mut self, viewport: Viewport, cursor_position: Point) {
        self.state.update(
            viewport.logical_size(),
            cursor_position,
            &mut self.renderer,
            &iced_glow::Theme::Dark,
            &renderer::Style {
                text_color: Color::WHITE,
            },
            &mut self.clipboard,
            &mut self.debug,
        );
    }

    #[inline]
    pub fn queue_event(&mut self, event: Event) {
        let state = &mut self.state;

        state.queue_event(event);
    }
}
