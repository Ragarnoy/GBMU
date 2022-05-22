use egui::Ui;
use winit::event_loop::EventLoopProxy;

use crate::{
    bios_configuration::BiosConfiguration,
    config::Mode,
    {custom_event::CustomEvent, windows::WindowType},
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

fn bios_configuration(bios_config: &mut BiosConfiguration, ui: &mut Ui) {
    if ui.checkbox(&mut bios_config.enable_dmg, "enable dmg bios").clicked() {
        if bios_config.enable_dmg{
            todo!("select a dmg bios file");
        } else {
            todo!("remove the dmg bios file");
        }
    }
    if bios_config.enable_dmg && ui.button("change dmg bios").clicked() {
        todo!("change the dmg bios file");
    }
    if ui.checkbox(&mut bios_config.enable_cbg, "enable cgb bios").clicked() {
        if bios_config.enable_cbg {
            todo!("select a cgb bios file");
        } else {
            todo!("remove the cgb bios file");
        }
    }
    if bios_config.enable_cbg && ui.button("change cgb bios").clicked() {
        todo!("change the cgb bios file");
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
