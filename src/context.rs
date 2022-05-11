mod debugger;
mod keybindings;

#[cfg(any(feature = "time_frame", feature = "debug_fps"))]
use crate::time_frame::TimeStat;
use crate::{
    config::Config, custom_event::CustomEvent, game::Game, image::load_image_to_frame,
    windows::WindowType,
};
use gb_lcd::{DrawEgui, GBPixels, GBWindow, PseudoPixels, PseudoWindow};
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
    pub main_window: GBPixels,
    pub joypad_config: Rc<RefCell<gb_joypad::Config>>,
    pub config: InternalConfig,
    pub event_proxy: EventLoopProxy<CustomEvent>,
    pub game: Option<Game>,
    #[cfg(any(feature = "time_frame", feature = "debug_fps"))]
    pub time_frame: TimeStat,
    #[cfg(any(feature = "time_frame", feature = "debug_fps"))]
    pub main_draw_instant: Instant,
    pub debugger_ctx: Option<debugger::Context>,
    pub keybindings_ctx: Option<keybindings::Context>,
}

#[derive(Default)]
pub struct InternalConfig {
    pub mode: Option<crate::config::Mode>,
    pub rom_file: Option<PathBuf>,
}

impl Context {
    pub fn new(main_window: GBPixels, event_proxy: EventLoopProxy<CustomEvent>) -> Self {
        Self {
            main_window,
            joypad_config: Rc::new(RefCell::new(keybindings::load_config())),
            config: InternalConfig::default(),
            event_proxy,
            game: None,
            #[cfg(any(feature = "time_frame", feature = "debug_fps"))]
            time_frame: TimeStat::default(),
            #[cfg(any(feature = "time_frame", feature = "debug_fps"))]
            main_draw_instant: Instant::now(),
            debugger_ctx: None,
            keybindings_ctx: None,
        }
    }

    pub fn load_config(&mut self, config: Config) {
        let config_file = config.rom.map(PathBuf::from);
        let reload_mode = self.config.mode != config.mode;
        let reload_file = self.config.rom_file != config_file;
        let open_debugger = config.debug;

        if reload_mode || reload_file {
            self.config.mode = config.mode;
            if let Some(file) = config_file.or_else(|| self.config.rom_file.clone()) {
                self.load(file)
            } else {
                log::warn!("Oh, I was expecting a file or something");
            }
        }

        if open_debugger && self.game.is_some() {
            self.event_proxy
                .send_event(CustomEvent::OpenWindow(WindowType::Debugger(Some(
                    config.breakpoints,
                ))))
                .expect("failed to send event to open debugger");
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
            WindowType::Debugger(breakpoints) => {
                if self.debugger_ctx.is_none() && self.game.is_some() {
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
                    self.debugger_ctx.replace(debugger::Context::new(
                        GBWindow::new(window),
                        self.event_proxy.clone(),
                        breakpoints,
                    ));
                }
            }
            WindowType::Keybindings => {
                if self.keybindings_ctx.is_none() {
                    let window = {
                        let size = LogicalSize::new(250.0, 250.0);
                        WindowBuilder::new()
                            .with_title("GBMU - Keybindings")
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
            WindowType::Debugger(_) => {
                self.debugger_ctx = None;
                self.debugger_ctx = None;
            }
            WindowType::Keybindings => {
                self.keybindings_ctx = None;
            }
        }
    }

    pub fn redraw(&mut self, window_id: WindowId) -> anyhow::Result<()> {
        if window_id == self.main_window.id() {
            self.redraw_main_window()
        } else if Some(window_id) == self.debugger_ctx.as_ref().map(|ctx| ctx.window.id()) {
            if let Some(game) = self.game.as_mut() {
                self.debugger_ctx.as_mut().unwrap().redraw_window(game)
            } else {
                log::warn!("Debugger need a game");
                Ok(())
            }
        } else if Some(window_id) == self.keybindings_ctx.as_ref().map(|ctx| ctx.window.id()) {
            self.keybindings_ctx.as_mut().unwrap().redraw_window()
        } else {
            panic!("unexpected window id {window_id:?}")
        }
    }

    pub fn process_window_event(&mut self, window_id: WindowId, event: WindowEvent) {
        if window_id == self.main_window.id() {
            self.process_main_window_event(event)
        } else if Some(window_id) == self.debugger_ctx.as_ref().map(|ctx| ctx.window.id()) {
            self.debugger_ctx
                .as_mut()
                .unwrap()
                .process_window_event(event)
        } else if Some(window_id) == self.keybindings_ctx.as_ref().map(|ctx| ctx.window.id()) {
            self.keybindings_ctx
                .as_mut()
                .unwrap()
                .process_window_event(event)
        } else {
            log::error!("unexpected window id {window_id:?} for event {event:?}")
        }
    }
}

impl Context {
    pub fn load(&mut self, file: PathBuf) {
        match Game::new(&file, self.joypad_config.clone(), false, self.config.mode) {
            Ok(game) => {
                self.game.replace(game);
                self.config.rom_file.replace(file);
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

    /// Reset the game context allowing the restart a game
    pub fn reset_game(&mut self, wanted_mode: Option<crate::config::Mode>) {
        if let Some(ref rom_file) = self.config.rom_file {
            let selected_mode = wanted_mode.or(self.config.mode);

            match Game::new(rom_file, self.joypad_config.clone(), false, selected_mode) {
                Ok(game) => {
                    self.game.replace(game);
                }
                Err(err) => {
                    log::error!(
                        "failed to reset the game from \"{}\", reason: {}",
                        rom_file.to_string_lossy(),
                        err
                    )
                }
            }
        }
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
        let main_pixels = &mut self.main_window.pixels;
        let main_context = &mut self.main_window.context;

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
        if self.main_window.context.on_event(&event) {
            return;
        }

        match event {
            WindowEvent::Resized(new_size) => {
                self.main_window.resize(new_size);
            }
            WindowEvent::ScaleFactorChanged {
                scale_factor,
                new_inner_size,
            } => {
                self.main_window.resize(*new_inner_size);
                self.main_window.context.scale_factor(scale_factor as f32);
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
