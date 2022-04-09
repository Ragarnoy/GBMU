use crate::Context;
#[cfg(feature = "debug_render")]
use crate::Game;
#[cfg(feature = "cgb")]
use crate::Opts;
#[cfg(feature = "debug_render")]
use native_dialog::FileDialog;

use gb_lcd::DrawEgui;

#[cfg(feature = "debug_fps")]
macro_rules! ui_fps {
    ($ui:expr, $context:expr, $fps:expr) => {
        $ui.add_space($ui.available_size().x - 50.0);
        $ui.label((format!("{:>7.2}", $fps)));
    };
}

pub fn draw_egui(
    context: &mut Context,
    #[cfg(feature = "cgb")] options: &mut Opts,
    #[cfg(feature = "debug_fps")] fps: f64,
) {
    context.windows.main.prepare_egui(|egui_ctx| {
        egui::containers::TopBottomPanel::top("Top menu").show(egui_ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.set_height(crate::constant::MENU_BAR_SIZE);
                ui.style_mut().override_text_style = Some(egui::TextStyle::Heading);
                // ui_file(ui, &mut context.custom_events);
                // ui_debug!(ui, context);
                // ui_settings!(ui, context, options, &mut context.custom_events);
                // ui.style_mut().override_text_style = None;
                #[cfg(feature = "debug_fps")]
                ui_fps!(ui, context, fps);
            });
        });
    })
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
