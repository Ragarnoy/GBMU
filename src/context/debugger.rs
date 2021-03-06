use gb_dbg::debugger::{options::DebuggerOptions, Debugger, DebuggerBuilder};
use gb_lcd::GBWindow;
use winit::{event::WindowEvent, event_loop::EventLoopProxy};

use crate::{custom_event::CustomEvent, game::Game, windows::WindowType};
use gb_lcd::{DrawEgui, PseudoPixels};

pub struct Context {
    pub window: GBWindow,
    pub debugger: Debugger,
    event_proxy: EventLoopProxy<CustomEvent>,
}

impl Context {
    pub fn new(
        window: GBWindow,
        event_proxy: EventLoopProxy<CustomEvent>,
        breakpoints: Option<Vec<String>>,
    ) -> Self {
        let mut builder = DebuggerBuilder::new();

        if let Some(breakpoints) = breakpoints {
            let options = DebuggerOptions {
                breakpoints,
                ..Default::default()
            };
            builder = builder.with_options(options);
        }

        let debugger = builder.build();

        Self {
            window,
            debugger,
            event_proxy,
        }
    }
}

/// Context impl for keybindings window
impl Context {
    pub(crate) fn redraw_window(&mut self, game: &mut Game) -> anyhow::Result<()> {
        let window = &mut self.window;
        let debugger = &mut self.debugger;

        window
            .context
            .prepare_egui(&window.window, |ctx| debugger.draw(ctx, game, None));

        window
            .render_with(|_encoder, _render_target, _context| Ok(()))
            .map_err(anyhow::Error::from)?;

        if self.debugger.reset_triggered {
            self.event_proxy
                .send_event(CustomEvent::ResetGame)
                .expect("cannot send reset game event");
            self.debugger.reset()
        }
        Ok(())
    }

    pub(crate) fn process_window_event(&mut self, event: WindowEvent) {
        let debugger_window = &mut self.window;
        if debugger_window.context.on_event(&event) {
            return;
        }

        match event {
            WindowEvent::ScaleFactorChanged { scale_factor, .. } => {
                debugger_window.context.scale_factor(scale_factor as f32)
            }
            WindowEvent::CloseRequested => self
                .event_proxy
                .send_event(CustomEvent::CloseWindow(WindowType::Debugger(None)))
                .unwrap(),
            _ => {}
        }
    }
}
