mod keybindings;

#[cfg(any(feature = "time_frame", feature = "debug_fps"))]
use crate::time_frame::TimeStat;
use crate::{
    config::Config,
    custom_event::CustomEvent,
    game::Game,
    image::load_image_to_frame,
    windows::{WindowType, Windows},
};
use gb_dbg::debugger::Debugger;
use gb_lcd::{DrawEgui, GBWindow, PseudoPixels, PseudoWindow};
#[cfg(any(feature = "time_frame", feature = "debug_fps"))]
use std::time::Instant;
use std::{cell::RefCell, path::PathBuf, rc::Rc};
use winit::{
    dpi::LogicalSize,
    event::{ElementState, WindowEvent},
    event_loop::{EventLoopProxy, EventLoopWindowTarget},
    window::{WindowBuilder, WindowId},
};

pub struct Context {
    pub windows: Windows,
    pub joypad_config: Rc<RefCell<gb_joypad::Config>>,
    pub config: Config,
    pub event_proxy: EventLoopProxy<CustomEvent>,
    pub game: Option<Game>,
    #[cfg(any(feature = "time_frame", feature = "debug_fps"))]
    pub time_frame: TimeStat,
    #[cfg(any(feature = "time_frame", feature = "debug_fps"))]
    pub main_draw_instant: Instant,
    pub debugger: Option<Debugger<Game>>,
    pub keybindings_ctx: Option<keybindings::Context>,
}

impl Context {
    pub fn new(windows: Windows, config: Config, event_proxy: EventLoopProxy<CustomEvent>) -> Self {
        Self {
            windows,
            // joypad_config: load_joypad_config(),
            joypad_config: Rc::new(RefCell::new(gb_joypad::Config::default())),
            config,
            event_proxy,
            game: None,
            #[cfg(any(feature = "time_frame", feature = "debug_fps"))]
            time_frame: TimeStat::default(),
            #[cfg(any(feature = "time_frame", feature = "debug_fps"))]
            main_draw_instant: Instant::now(),
            debugger: None,
            keybindings_ctx: None,
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
                            .with_title("GBMU - Cpu Debugger")
                            .with_inner_size(size)
                            .with_resizable(false)
                            .build(event_loop)
                            .expect("cannot build debugger window")
                    };
                    self.windows.debugger.replace(GBWindow::new(window));
                    self.debugger
                        .replace(gb_dbg::debugger::DebuggerBuilder::new().build());
                }
            }
            WindowType::Keybindings => {
                if self.keybindings_ctx.is_none() {
                    let window = {
                        let size = LogicalSize::new(250 as f64, 250 as f64);
                        WindowBuilder::new()
                            .with_title("GBMU - Keybindings")
                            .with_resizable(false)
                            .with_inner_size(size)
                            .build(event_loop)
                            .expect("cannot build keybinding window")
                    };
                    self.keybindings_ctx.replace(keybindings::Context::new(
                        GBWindow::new(window),
                        self.joypad_config.clone(),
                        self.event_proxy.clone(),
                    ));
                }
            }
        }
    }

    pub fn close_window(&mut self, window_type: WindowType) {
        match window_type {
            WindowType::Debugger => {
                self.windows.debugger = None;
                self.debugger = None;
            }
            WindowType::Keybindings => {
                self.keybindings_ctx = None;
            }
        }
    }

    pub fn redraw(&mut self, window_id: WindowId) -> anyhow::Result<()> {
        if window_id == self.windows.main.id() {
            self.redraw_main_window()
        } else if Some(window_id) == self.windows.debugger.as_ref().map(|win| win.id()) {
            self.redraw_debugger_window()
        } else if Some(window_id) == self.keybindings_ctx.as_ref().map(|ctx| ctx.window.id()) {
            self.keybindings_ctx
                .as_mut()
                .unwrap()
                .redraw_keybindings_window()
        } else {
            panic!("unexpected window id {window_id:?}")
        }
    }

    pub fn process_window_event(&mut self, window_id: WindowId, event: WindowEvent) {
        if window_id == self.windows.main.id() {
            self.process_main_window_event(event)
        } else if Some(window_id) == self.windows.debugger.as_ref().map(|win| win.id()) {
            self.process_debugger_window_event(event)
        } else if Some(window_id) == self.keybindings_ctx.as_ref().map(|ctx| ctx.window.id()) {
            self.keybindings_ctx
                .as_mut()
                .unwrap()
                .process_keybindings_window_event(event)
        } else {
            log::error!("unexpected window id {window_id:?} for event {event:?}")
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
        crate::ui::draw_egui(
            self,
            #[cfg(feature = "debug_fps")]
            self.time_frame.instant_fps(),
        );
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

        #[cfg(any(feature = "time_frame", feature = "debug_fps"))]
        {
            self.time_frame.add_sample(self.main_draw_instant.elapsed());
            self.main_draw_instant = std::time::Instant::now();
        }

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
            _ => {}
        }
    }
}

/// Context impl for debugger window
impl Context {
    fn redraw_debugger_window(&mut self) -> anyhow::Result<()> {
        let window = self.windows.debugger.as_mut().unwrap();
        let debugger = self.debugger.as_mut().unwrap();
        let game = self.game.as_mut().unwrap();

        window
            .context
            .prepare_egui(&window.window, |ctx| debugger.draw(ctx, game, None));

        window
            .render_with(|_encoder, _render_target, _context| Ok(()))
            .map_err(anyhow::Error::from)
    }

    fn process_debugger_window_event(&mut self, event: WindowEvent) {
        let debugger_window = self.windows.debugger.as_mut().unwrap();
        if debugger_window.context.on_event(&event) {
            return;
        }

        match event {
            WindowEvent::ScaleFactorChanged { scale_factor, .. } => {
                debugger_window.context.scale_factor(scale_factor as f32)
            }
            WindowEvent::CloseRequested => self
                .event_proxy
                .send_event(CustomEvent::CloseWindow(WindowType::Debugger))
                .unwrap(),
            _ => {}
        }
    }
}
