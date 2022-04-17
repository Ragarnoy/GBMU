use std::{cell::RefCell, rc::Rc};

use gb_joypad::{Config, InputType};
use gb_lcd::{DrawEgui, GBWindow, PseudoPixels};
use winit::{event::WindowEvent, event_loop::EventLoopProxy};

use crate::{custom_event::CustomEvent, windows::WindowType};

pub(crate) struct Context {
    pub window: GBWindow,
    listening: Option<InputType>,
    config: Rc<RefCell<Config>>,
    event_proxy: EventLoopProxy<CustomEvent>,
}

impl Context {
    pub fn new(
        window: GBWindow,
        config: Rc<RefCell<Config>>,
        event_proxy: EventLoopProxy<CustomEvent>,
    ) -> Self {
        Self {
            window,
            listening: None,
            config,
            event_proxy,
        }
    }
}

/// Context impl for keybindings window
impl Context {
    pub(crate) fn redraw_keybindings_window(&mut self) -> anyhow::Result<()> {
        let window = self.window;
        let config = self.config.borrow_mut();

        window.context.prepare_egui(&window.window, |ctx| {
            crate::ui::window::keybindings::draw_window(ctx, &mut config)
        });

        window
            .render_with(|_encoder, _render_target, _context| Ok(()))
            .map_err(anyhow::Error::from)
    }

    pub(crate) fn process_keybindings_window_event(&mut self, event: WindowEvent) {
        let window = self.window;
        if window.context.on_event(&event) {
            return;
        }

        match event {
            WindowEvent::Resized(size) => window.resize(size),
            WindowEvent::ScaleFactorChanged {
                scale_factor,
                new_inner_size,
            } => {
                window.context.scale_factor(scale_factor as f32);
                window.resize(*new_inner_size);
            }
            WindowEvent::CloseRequested => self
                .event_proxy
                .send_event(CustomEvent::CloseWindow(WindowType::Keybindings))
                .unwrap(),
            _ => {}
        }
    }
}
