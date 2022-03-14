#[cfg(feature = "debug_render")]
use crate::Game;
#[cfg(feature = "cgb")]
use crate::Opts;
use crate::{custom_event::CustomEvent, Context};
use egui::Ui;
use gb_dbg::{DEBUGGER_HEIGHT, DEBUGGER_WIDTH};
use gb_lcd::{render, window::GBWindow};
#[cfg(feature = "debug_render")]
use gb_ppu::{
    SPRITE_LIST_RENDER_HEIGHT, SPRITE_LIST_RENDER_WIDTH, SPRITE_RENDER_HEIGHT, SPRITE_RENDER_WIDTH,
    TILEMAP_DIM, TILESHEET_HEIGHT, TILESHEET_WIDTH,
};
use native_dialog::FileDialog;

macro_rules! replace_windows {
    ($context:expr, $name:ident, $window:expr) => {
        $context.windows.$name.replace($window)
    };
}

macro_rules! ui_debug {
    ($ui:expr, $context:expr) => {
        $ui.menu_button("🔧", |ui| {
            ui.style_mut().override_text_style = None;
            if ui.button("Cpu").clicked() && $context.windows.debug.is_none() {
                replace_windows!($context, debug, new_debug_window(&$context.video));
            }
            #[cfg(feature = "debug_render")]
            {
                if ui.button("tilesheet").clicked() && $context.windows.tilesheet.is_none() {
                    replace_windows!($context, tilesheet, new_tilesheep_window(&$context.video));
                }
                if ui.button("tilemap").clicked() && $context.windows.tilemap.is_none() {
                    replace_windows!($context, tilemap, new_tilemap_window(&$context.video));
                }
                if ui.button("objects").clicked() && $context.windows.oam.is_none() {
                    replace_windows!($context, oam, new_objects_window(&$context.video));
                }
            }
        });
    };
}

macro_rules! ui_settings {
    ($ui:expr, $context:expr, $opts:expr, $events:expr) => {
        $ui.menu_button("⚙", |ui| {
            ui.style_mut().override_text_style = None;
            if ui.button("Input").clicked() && $context.windows.input.is_none() {
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
            ui.separator();
            #[cfg(feature = "cgb")]
            {
                ui.radio_value(&mut $opts.mode, None, "auto");
                if ui
                    .radio_value(
                        &mut $opts.mode,
                        Some(crate::Mode::Classic),
                        crate::Mode::Classic.to_string(),
                    )
                    .clicked()
                {
                    $events.push(CustomEvent::ChangedMode(crate::Mode::Classic));
                }
                if ui
                    .radio_value(
                        &mut $opts.mode,
                        Some(crate::Mode::Color),
                        crate::Mode::Color.to_string(),
                    )
                    .clicked()
                {
                    $events.push(CustomEvent::ChangedMode(crate::Mode::Color));
                }
            }
        });
    };
}

#[cfg(feature = "debug_fps")]
macro_rules! ui_fps {
    ($ui:expr, $context:expr, $fps:expr) => {
        $ui.add_space($ui.available_size().x - 50.0);
        $ui.label((format!("{:>7.2}", $fps)));
    };
}

pub fn draw_egui<const WIDTH: usize, const HEIGHT: usize>(
    context: &mut Context<WIDTH, HEIGHT>,
    #[cfg(feature = "cgb")] options: &mut Opts,
    #[cfg(feature = "debug_fps")] fps: f64,
) -> Vec<CustomEvent> {
    let mut events = Vec::new();
    egui::containers::TopBottomPanel::top("Top menu").show(context.windows.main.egui_ctx(), |ui| {
        egui::menu::bar(ui, |ui| {
            ui.set_height(render::MENU_BAR_SIZE);
            ui.style_mut().override_text_style = Some(egui::TextStyle::Heading);
            ui_file(ui, &mut events);
            ui_debug!(ui, context);
            ui_settings!(ui, context, options, events);
            ui.style_mut().override_text_style = None;
            #[cfg(feature = "debug_fps")]
            ui_fps!(ui, context, fps);
        });
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
                        ui.menu_button("bg/win", |ui| {
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
                    ui.menu_button("mode", |ui| {
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
    ui.menu_button("💾", |ui| {
        ui.style_mut().override_text_style = None;
        if ui.button("Load").clicked() {
            let file = FileDialog::new()
                .set_location(
                    &std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("/")),
                )
                .add_filter("rom", &crate::constant::PREFERED_ROM_EXTS)
                .show_open_single_file();
            log::debug!("picked romfile: {:?}", file);
            if let Ok(Some(path)) = file {
                events.push(CustomEvent::LoadFile(path));
            }
        }
        #[cfg(feature = "save_state")]
        {
            use crate::context::game_root_config_path;

            ui.separator();
            if ui.button("save as").clicked() {
                let file = FileDialog::new()
                    .set_location(&game_root_config_path())
                    .add_filter("save state", &crate::constant::PREFERED_SAVE_STATE_EXT)
                    .show_save_single_file();
                log::debug!("picked name for 'save state' file: {:?}", file);
                if let Ok(Some(path)) = file {
                    events.push(CustomEvent::SaveState(path));
                }
            }
            if ui.button("load save").clicked() {
                let file = FileDialog::new()
                    .set_location(&game_root_config_path())
                    .add_filter("save state", &crate::constant::PREFERED_SAVE_STATE_EXT)
                    .show_open_single_file();
                log::debug!("picked a file to load the state from: {:?}", file);
                if let Ok(Some(path)) = file {
                    events.push(CustomEvent::LoadState(path));
                }
            }
        }
    });
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

#[cfg(feature = "debug_render")]
fn new_objects_window(video: &sdl2::VideoSubsystem) -> crate::windows::OAMConfig {
    use crate::windows::OAMConfig;

    let bar_pixels_size = GBWindow::dots_to_pixels(video, render::MENU_BAR_SIZE)
        .expect("Error while computing bar size");
    let mut oam = GBWindow::new(
        "ppu oam",
        (
            SPRITE_RENDER_WIDTH as u32,
            SPRITE_RENDER_HEIGHT as u32 + bar_pixels_size,
        ),
        true,
        video,
    )
    .expect("Error while building oam window");
    oam.sdl_window_mut()
        .set_minimum_size(
            SPRITE_RENDER_WIDTH as u32,
            SPRITE_RENDER_HEIGHT as u32 + bar_pixels_size,
        )
        .expect("Failed to configure oam window");
    OAMConfig {
        window: oam,
        viewport: render::RenderImage::<
            SPRITE_RENDER_WIDTH, SPRITE_RENDER_HEIGHT
            >::with_bar_size(bar_pixels_size as f32),
        list: render::RenderImage::<
            SPRITE_LIST_RENDER_WIDTH,SPRITE_LIST_RENDER_HEIGHT
            >::with_bar_size(bar_pixels_size as f32),
        display_list: false,
        invert_color: false,
    }
}

#[cfg(feature = "debug_render")]
fn new_tilemap_window(
    video: &sdl2::VideoSubsystem,
) -> (
    GBWindow,
    render::RenderImage<TILEMAP_DIM, TILEMAP_DIM>,
    bool,
) {
    let bar_pixels_size = GBWindow::dots_to_pixels(video, render::MENU_BAR_SIZE)
        .expect("Error while computing bar size");
    let mut tilemap = GBWindow::new(
        "ppu tilemap",
        (TILEMAP_DIM as u32, TILEMAP_DIM as u32 + bar_pixels_size),
        true,
        video,
    )
    .expect("Error while building tilemap window");
    tilemap
        .sdl_window_mut()
        .set_minimum_size(TILEMAP_DIM as u32, TILEMAP_DIM as u32 + bar_pixels_size)
        .expect("Failed to configure tilemap window");
    (
        tilemap,
        render::RenderImage::<TILEMAP_DIM, TILEMAP_DIM>::with_bar_size(bar_pixels_size as f32),
        false,
    )
}

#[cfg(feature = "debug_render")]
fn new_tilesheep_window(
    video: &sdl2::VideoSubsystem,
) -> (
    GBWindow,
    render::RenderImage<TILESHEET_WIDTH, TILESHEET_HEIGHT>,
) {
    let mut tilesheet = GBWindow::new(
        "ppu tilesheet",
        (TILESHEET_WIDTH as u32, TILESHEET_HEIGHT as u32),
        true,
        video,
    )
    .expect("Error while building tilesheet window");
    tilesheet
        .sdl_window_mut()
        .set_minimum_size(TILESHEET_WIDTH as u32, TILESHEET_HEIGHT as u32)
        .expect("Failed to configure tilesheet window");
    (
        tilesheet,
        render::RenderImage::<TILESHEET_WIDTH, TILESHEET_HEIGHT>::with_bar_size(0.0),
    )
}
