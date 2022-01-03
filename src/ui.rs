#[cfg(feature = "debug_render")]
use crate::Game;
use crate::{custom_event::CustomEvent, Context};
use egui::Ui;
use gb_dbg::{DEBUGGER_HEIGHT, DEBUGGER_WIDTH};
use gb_lcd::{render, window::GBWindow};
use native_dialog::FileDialog;

#[cfg(feature = "debug_render")]
macro_rules! load_tilesheet_windows {
    ($context:expr) => {
        use gb_ppu::{TILESHEET_HEIGHT, TILESHEET_WIDTH};

        let mut tilesheet = GBWindow::new(
            "ppu tilesheet",
            (TILESHEET_WIDTH as u32, TILESHEET_HEIGHT as u32),
            true,
            &$context.video,
        )
        .expect("Error while building tilesheet window");
        tilesheet
            .sdl_window_mut()
            .set_minimum_size(TILESHEET_WIDTH as u32, TILESHEET_HEIGHT as u32)
            .expect("Failed to configure tilesheet window");
        $context.windows.tilesheet.replace((
            tilesheet,
            render::RenderImage::<TILESHEET_WIDTH, TILESHEET_HEIGHT>::with_bar_size(0.0),
        ))
    };
}

#[cfg(feature = "debug_render")]
macro_rules! load_tilemap_windows {
    ($context:expr) => {
        use gb_ppu::TILEMAP_DIM;

        let bar_pixels_size = GBWindow::dots_to_pixels(&$context.video, render::MENU_BAR_SIZE)
            .expect("Error while computing bar size");
        let mut tilemap = GBWindow::new(
            "ppu tilemap",
            (TILEMAP_DIM as u32, TILEMAP_DIM as u32 + bar_pixels_size),
            true,
            &$context.video,
        )
        .expect("Error while building tilemap window");
        tilemap
            .sdl_window_mut()
            .set_minimum_size(TILEMAP_DIM as u32, TILEMAP_DIM as u32 + bar_pixels_size)
            .expect("Failed to configure tilemap window");
        $context.windows.tilemap.replace((
            tilemap,
            render::RenderImage::<TILEMAP_DIM, TILEMAP_DIM>::with_bar_size(bar_pixels_size as f32),
            false,
        ))
    };
}

#[cfg(feature = "debug_render")]
macro_rules! load_objects_windows {
    ($context:expr) => {
        use gb_ppu::{
            SPRITE_LIST_RENDER_HEIGHT, SPRITE_LIST_RENDER_WIDTH, SPRITE_RENDER_HEIGHT,
            SPRITE_RENDER_WIDTH,
        };

        let bar_pixels_size = GBWindow::dots_to_pixels(&$context.video, render::MENU_BAR_SIZE)
            .expect("Error while computing bar size");
        let mut oam = GBWindow::new(
            "ppu oam",
            (
                SPRITE_RENDER_WIDTH as u32,
                SPRITE_RENDER_HEIGHT as u32 + bar_pixels_size,
            ),
            true,
            &$context.video,
        )
        .expect("Error while building oam window");
        oam.sdl_window_mut()
            .set_minimum_size(
                SPRITE_RENDER_WIDTH as u32,
                SPRITE_RENDER_HEIGHT as u32 + bar_pixels_size,
            )
            .expect("Failed to configure oam window");
        $context.windows.oam.replace(crate::windows::OAMConfig {
                                                    window: oam,
                                                    viewport: render::RenderImage::<
                                                            SPRITE_RENDER_WIDTH,
                                                            SPRITE_RENDER_HEIGHT,
                                                        >::with_bar_size(
                                                    bar_pixels_size as f32
                                                    ),
                                                    list: render::RenderImage::<
                                                        SPRITE_LIST_RENDER_WIDTH,
                                                        SPRITE_LIST_RENDER_HEIGHT,
                                                        >::with_bar_size(bar_pixels_size as f32),
                                                    display_list: false,
                                                    invert_color: false,
                                                })
    };
}

macro_rules! ui_debug {
    ($ui:expr, $context:expr) => {
        egui::menu::menu($ui, "Debug", |ui| {
            if ui.button("Cpu").clicked() && $context.windows.debug.is_none() {
                $context
                    .windows
                    .debug
                    .replace(new_debug_window(&$context.video));
            }
            #[cfg(feature = "debug_render")]
            {
                if ui.button("tilesheet").clicked() && $context.windows.tilesheet.is_none() {
                    load_tilesheet_windows!($context);
                }
                if ui.button("tilemap").clicked() && $context.windows.tilemap.is_none() {
                    load_tilemap_windows!($context);
                }
                if ui.button("objects").clicked() && $context.windows.oam.is_none() {
                    load_objects_windows!($context);
                }
            }
        });
    };
}

macro_rules! ui_settings {
    ($ui:expr, $context:expr) => {
        if $ui.button("Input").clicked() && $context.windows.input.is_none() {
            $context.windows.input.replace(
                GBWindow::new(
                    "GBMU Input Settings",
                    (
                        GBWindow::dots_to_pixels(&$context.video, 250.0)
                            .expect("error while computing window size"),
                        GBWindow::dots_to_pixels(&$context.video, 250.0)
                            .expect("error while computing window size"),
                    ),
                    false,
                    &$context.video,
                )
                .expect("Error while building input window"),
            );
        }
    };
}

pub fn draw_egui<const WIDTH: usize, const HEIGHT: usize>(
    context: &mut Context<WIDTH, HEIGHT>,
) -> Vec<CustomEvent> {
    let mut events = Vec::new();
    egui::containers::TopBottomPanel::top("Top menu").show(context.windows.main.egui_ctx(), |ui| {
        egui::menu::bar(ui, |ui| {
            ui.set_height(render::MENU_BAR_SIZE);
            ui_file(ui, &mut events);
            ui_debug!(ui, context);
            ui_settings!(ui, context);
        })
    });
    events
}

#[cfg(feature = "debug_render")]
pub fn draw_ppu_debug_ui<const WIDTH: usize, const HEIGHT: usize>(
    context: &mut Context<WIDTH, HEIGHT>,
    game: &mut Option<Game>,
) {
    if let Some((ref mut tilemap_window, ref mut display, ref mut display_window)) =
        context.windows.tilemap
    {
        tilemap_window
            .start_frame()
            .expect("Fail at the start for the tilemap window");
        if let Some(ref mut game) = game {
            egui::containers::TopBottomPanel::top("Top menu").show(
                tilemap_window.egui_ctx(),
                |ui| {
                    egui::menu::bar(ui, |ui| {
                        ui.set_height(render::MENU_BAR_SIZE);
                        egui::menu::menu(ui, "bg/win", |ui| {
                            if ui.button("background").clicked() {
                                *display_window = false;
                            }
                            if ui.button("window").clicked() {
                                *display_window = true;
                            }
                        });
                    })
                },
            );
            display.update_render(&game.ppu.tilemap_image(*display_window));
        }
        display.draw();
        tilemap_window
            .end_frame()
            .expect("Fail at the end for the tilemap window");
    }

    if let Some((ref mut tilesheet_window, ref mut display)) = context.windows.tilesheet {
        tilesheet_window
            .start_frame()
            .expect("Fail at the start for the tilesheet window");
        if let Some(ref mut game) = game {
            display.update_render(&game.ppu.tilesheet_image());
        }
        display.draw();
        tilesheet_window
            .end_frame()
            .expect("Fail at the end for the tilesheet window");
    }

    if let Some(ref mut cfg) = context.windows.oam {
        cfg.window
            .start_frame()
            .expect("Fail at the start for the oam window");
        if let Some(ref mut game) = game {
            egui::containers::TopBottomPanel::top("Top menu").show(cfg.window.egui_ctx(), |ui| {
                egui::menu::bar(ui, |ui| {
                    ui.set_height(render::MENU_BAR_SIZE);
                    egui::menu::menu(ui, "mode", |ui| {
                        if ui.button("viewport").clicked() {
                            cfg.display_list = false;
                        }
                        if ui.button("list").clicked() {
                            cfg.display_list = true;
                        }
                    });
                    ui.checkbox(&mut cfg.invert_color, "invert")
                })
            });
            if !cfg.display_list {
                cfg.viewport
                    .update_render(&game.ppu.sprites_image(cfg.invert_color));
                cfg.viewport.draw();
            } else {
                cfg.list
                    .update_render(&game.ppu.sprites_list_image(cfg.invert_color));
                cfg.list.draw();
            }
        }
        cfg.window
            .end_frame()
            .expect("Fail at the end for the oam window");
    }
}

fn ui_file(ui: &mut Ui, events: &mut Vec<CustomEvent>) {
    if ui.button("Load").clicked() {
        let file = FileDialog::new()
            .set_location(
                &std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("/")),
            )
            .add_filter("rom", &["gb", "gbc", "rom"])
            .show_open_single_file();
        log::debug!("picked file: {:?}", file);
        if let Ok(Some(path)) = file {
            events.push(CustomEvent::LoadFile(path));
        }
    }
}

pub fn new_debug_window(video: &sdl2::VideoSubsystem) -> GBWindow {
    GBWindow::new(
        "GBMU Debug",
        (DEBUGGER_WIDTH as u32, DEBUGGER_HEIGHT as u32),
        false,
        video,
    )
    .expect("Error while building debug window")
}
