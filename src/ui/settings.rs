use std::path::PathBuf;

use egui::Ui;
use native_dialog::FileDialog;
use winit::event_loop::EventLoopProxy;

use crate::{
    {custom_event::CustomEvent, windows::WindowType},
    bios_configuration::BiosConfiguration,
    config::Mode,
};

pub(crate) fn draw_ui(
    ui: &mut Ui,
    event_proxy: &EventLoopProxy<CustomEvent>,
    bios_config: &mut BiosConfiguration,
    mode: &mut Option<Mode>,
) {
    ui.menu_button("Settings", |ui| {
        ui.style_mut().override_text_style = None;
        if ui.button("Input").clicked() {
            event_proxy
                .send_event(CustomEvent::OpenWindow(WindowType::Keybindings))
                .expect("cannot send open keybindings window event");
        }

        ui.separator();
        bios_configuration(bios_config, ui);

        ui.separator();
        mode_settings(event_proxy, mode, ui);
    });
}

const PREFERRED_BIOS_EXTS: [&str; 2] = ["bios", "bin"];

fn bios_configuration(bios_config: &mut BiosConfiguration, ui: &mut Ui) {
    if ui
        .checkbox(&mut bios_config.enable_dmg, "enable dmg bios")
        .clicked() && bios_config.enable_dmg && bios_config.dmg_bios_file.is_none() {
        select_bios_file(&mut bios_config.dmg_bios_file);
    }
    if bios_config.enable_dmg && ui.button("change dmg bios").clicked() {
        select_bios_file(&mut bios_config.dmg_bios_file);
    }
    if let Some(ref path) = bios_config.dmg_bios_file {
        ui.label(format!("dmg bios: {}", path.to_string_lossy()));
        ui.label(path.to_string_lossy().to_string());
    }
    if ui
        .checkbox(&mut bios_config.enable_cbg, "enable cgb bios")
        .clicked() && bios_config.enable_cbg && bios_config.cgb_bios_file.is_none() {
        select_bios_file(&mut bios_config.cgb_bios_file);
    }
    if bios_config.enable_cbg && ui.button("change cgb bios").clicked() {
        select_bios_file(&mut bios_config.cgb_bios_file);
    }
    if let Some(ref path) = bios_config.cgb_bios_file {
        ui.label(format!("cgb bios: {}", path.to_string_lossy()));
        ui.label(path.to_string_lossy().to_string());
    }
}

fn select_bios_file(value: &mut Option<PathBuf>) {
    let file = FileDialog::new()
        .set_location(&std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("/")))
        .add_filter("bios", &PREFERRED_BIOS_EXTS)
        .show_open_single_file();

    log::debug!("picked bios file {file:?}");
    if let Ok(Some(path)) = file {
        value.replace(path);
    }
}

fn mode_settings(event_proxy: &EventLoopProxy<CustomEvent>, mode: &mut Option<Mode>, ui: &mut Ui) {
    if ui.radio_value(mode, None, "auto").clicked() {
        event_proxy
            .send_event(CustomEvent::ChangedMode(None))
            .unwrap();
    }
    if ui.radio_value(mode, Some(Mode::Classic), "dmg").clicked() {
        event_proxy
            .send_event(CustomEvent::ChangedMode(Some(Mode::Classic)))
            .unwrap();
    }
    if ui.radio_value(mode, Some(Mode::Color), "cgb").clicked() {
        event_proxy
            .send_event(CustomEvent::ChangedMode(Some(Mode::Color)))
            .unwrap();
    }
}
