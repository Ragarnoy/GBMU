use std::path::PathBuf;

use crate::{
    config::Config,
    custom_event::CustomEvent,
    game::Game,
    image::load_image_to_frame,
    windows::{WindowType, Windows},
};
use gb_lcd::{DrawEgui, GBWindow, PseudoPixels, PseudoWindow};
use winit::{
    dpi::LogicalSize,
    event::{ElementState, WindowEvent},
    event_loop::{EventLoopProxy, EventLoopWindowTarget},
    window::{WindowBuilder, WindowId},
};

pub struct Context {
    pub windows: Windows,
    pub joypad_config: gb_joypad::Config,
    pub config: Config,
    pub event_proxy: EventLoopProxy<CustomEvent>,
    pub game: Option<Game>,
}

impl Context {
    pub fn new(windows: Windows, config: Config, event_proxy: EventLoopProxy<CustomEvent>) -> Self {
        Self {
            windows,
            // joypad_config: load_joypad_config(),
            joypad_config: gb_joypad::Config::default(),
            config,
            event_proxy,
            game: None,
        }
    }
}

impl Context {
    pub fn open_window(
        &mut self,
        window_type: WindowType,
        event_loop: &EventLoopWindowTarget<CustomEvent>,
    ) {
        match window_type {
            WindowType::Debugger => {
                if self.windows.debugger.is_none() {
                    let window = {
                        let size =
                            LogicalSize::new(gb_dbg::DEBUGGER_WIDTH, gb_dbg::DEBUGGER_HEIGHT);
                        WindowBuilder::new()
                            .with_title("cpu debugger")
                            .with_inner_size(size)
                            .with_resizable(false)
                            .build(event_loop)
                            .expect("cannot build debugger window")
                    };
                    self.windows.debugger.replace(GBWindow::new(window));
                }
            }
            _ => todo!("cannot currently open window {window_type:?}"),
        }
    }

    pub fn redraw(&mut self, window_id: WindowId) -> anyhow::Result<()> {
        if window_id == self.windows.main.id() {
            self.redraw_main_window()
        } else {
            panic!("unexpected window id {window_id:?}")
        }
    }

    pub fn process_window_event(&mut self, window_id: WindowId, event: WindowEvent) {
        if window_id == self.windows.main.id() {
            self.process_main_window_event(event)
        } else {
            panic!("unexpected window id {window_id:?}")
        }
    }
}

impl Context {
    pub fn load(&mut self, file: PathBuf) {
        match Game::new(
            &file,
            self.joypad_config.clone(),
            false,
            #[cfg(feature = "cgb")]
            self.config.mode,
        ) {
            Ok(game) => {
                self.game.replace(game);
            }
            Err(err) => {
                log::error!(
                    "Failed to load rom file \"{}\": {}",
                    file.to_string_lossy(),
                    err
                );
            }
        };
    }
}

/// Context impl for main window
impl Context {
    pub fn redraw_main_window(&mut self) -> anyhow::Result<()> {
        crate::ui::draw_egui(self);
        let main_pixels = &mut self.windows.main.pixels;
        let main_context = &mut self.windows.main.context;

        if let Some(ref game) = self.game {
            let image = game.ppu.pixels();
            let frame = main_pixels.get_frame();
            load_image_to_frame(image, frame);
        }
        main_pixels.render_with(|encoder, render_target, context| {
            // Render pixels buffer
            context.scaling_renderer.render(encoder, render_target);

            main_context.render_egui(
                encoder,
                render_target,
                &gb_lcd::RenderContext::from(context),
            )?;

            Ok(())
        })?;

        Ok(())
    }

    fn process_main_window_event(&mut self, event: WindowEvent) {
        if self.windows.main.context.on_event(&event) {
            return;
        }

        match event {
            WindowEvent::Resized(new_size) => {
                self.windows.main.resize(new_size);
            }
            WindowEvent::ScaleFactorChanged {
                scale_factor,
                new_inner_size,
            } => {
                self.windows.main.resize(*new_inner_size);
                self.windows.main.context.scale_factor(scale_factor as f32);
            }
            WindowEvent::CloseRequested => self
                .event_proxy
                .send_event(CustomEvent::Quit)
                .expect("cannot send quit event"),
            WindowEvent::DroppedFile(path) => self
                .event_proxy
                .send_event(CustomEvent::LoadFile(path))
                .expect("cannot send load file event"),
            WindowEvent::KeyboardInput { input, .. } => {
                use gb_joypad::KeyEntry;

                let pressed = input.state == ElementState::Pressed;
                let key = KeyEntry::from(input);
                if let Some(ref mut game) = self.game {
                    game.joypad.borrow_mut().on_key_event(key, pressed);
                }
            }
            WindowEvent::CursorMoved { .. }
            | WindowEvent::CursorEntered { .. }
            | WindowEvent::CursorLeft { .. }
            | WindowEvent::MouseInput { .. }
            | WindowEvent::AxisMotion { .. }
            | WindowEvent::Moved(_)
            | WindowEvent::Focused(_)
            | WindowEvent::ReceivedCharacter(_)
            | WindowEvent::ModifiersChanged(_) => {
                // log::debug!("ignore main window event {event:?}")
            }
            _ => todo!("process main window event {event:?}"),
        }
    }
}
