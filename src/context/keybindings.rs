use std::ops::Deref;
use std::{cell::RefCell, rc::Rc};

use egui::{Direction, Layout, Separator, Ui};
use winit::{event::WindowEvent, event_loop::EventLoopProxy};

use gb_joypad::{input::INPUT_LIST, Config, InputType, KeyEntry};
use gb_lcd::{DrawEgui, GBWindow, PseudoPixels};

use crate::{custom_event::CustomEvent, windows::WindowType};

pub struct Context {
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
    pub(crate) fn redraw_window(&mut self) -> anyhow::Result<()> {
        let window = &mut self.window;
        let config = &self.config;

        window.context.prepare_egui(&window.window, |ctx| {
            Context::draw_window(ctx, config, &mut self.listening)
        });

        window
            .render_with(|_encoder, _render_target, _context| Ok(()))
            .map_err(anyhow::Error::from)
    }

    pub(crate) fn process_window_event(&mut self, event: WindowEvent) {
        let window = &mut self.window;
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
            WindowEvent::KeyboardInput { input, .. } => {
                if let Some(input_type) = self.listening {
                    let new_key = KeyEntry::from(input);

                    self.config
                        .borrow_mut()
                        .update_keybinding(input_type, new_key);
                    self.listening = None;
                }
            }
            WindowEvent::CloseRequested => self
                .event_proxy
                .send_event(CustomEvent::CloseWindow(WindowType::Keybindings))
                .unwrap(),
            _ => {}
        }
    }
}

impl Context {
    fn draw_window(
        ctx: &egui::Context,
        config: &Rc<RefCell<Config>>,
        listening: &mut Option<InputType>,
    ) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let height = ui.available_size().y;
            egui::ScrollArea::vertical()
                .max_height(height - 50.0)
                .show(ui, |ui| {
                    ui.set_height(height - 60.0);
                    for input_type in INPUT_LIST.iter() {
                        ui.horizontal(|ui| {
                            if let Some(listened) = listening {
                                Context::input_row(
                                    ui,
                                    config,
                                    input_type,
                                    listened == input_type,
                                    listening,
                                );
                            } else {
                                Context::input_row(ui, config, input_type, false, listening);
                            }
                        });
                    }
                });
            ui.vertical(|ui| {
                ui.vertical_centered(|ui| {
                    ui.add(Separator::default().horizontal().spacing(30.0));
                    if ui.button("reset   ⟲").clicked() {
                        *listening = None;
                        *(config.borrow_mut()) = Config::default();
                    }
                });
            });
        });
    }

    fn input_row(
        ui: &mut Ui,
        config: &Rc<RefCell<Config>>,
        input_type: &InputType,
        force_empty: bool,
        listening: &mut Option<InputType>,
    ) {
        ui.columns(3, |ui| {
            ui[0].label(format!("{:?}:", input_type));
            ui[1].with_layout(
                Layout::centered_and_justified(Direction::LeftToRight),
                |ui| {
                    if force_empty {
                        ui.label("---");
                    } else {
                        match config.borrow().get_key_entry(input_type) {
                            Some(entry) => ui.label(entry.name()),
                            None => ui.label("---"),
                        };
                    }
                },
            );
            ui[2].with_layout(Layout::right_to_left(), |ui| {
                if !force_empty && ui.button("⚙").clicked() {
                    *listening = Some(*input_type);
                } else if force_empty && ui.button("❌").clicked() {
                    *listening = None;
                }
            });
        });
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        crate::path::create_root_config_path().expect("failed to create config directory");
        let keybindings_config_path = crate::path::keybinding_path();

        log::info!(
            "saving keybindings configuration to {}",
            keybindings_config_path.to_string_lossy()
        );

        let keybindings_config_file = std::fs::File::create(keybindings_config_path)
            .expect("cannot create file for keybindings");

        serde_yaml::to_writer(keybindings_config_file, self.config.borrow().deref())
            .expect("cannot save keybindings config file");
    }
}
