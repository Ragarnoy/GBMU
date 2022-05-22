use egui::Ui;
use winit::event_loop::EventLoopProxy;

use crate::config::Mode;
use crate::{custom_event::CustomEvent, windows::WindowType};

pub(crate) fn draw_ui(
    ui: &mut Ui,
    event_proxy: &EventLoopProxy<CustomEvent>,
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
        bios_configuration(ui);

        ui.separator();
        mode_settings(event_proxy, mode, ui);
    });
}

fn bios_configuration(ui: &mut Ui) {
    let mut dmg_bios = false;
    let mut cgb_bios = false;
    if ui.checkbox(&mut dmg_bios, "enable dmg bios").clicked() {
        if dmg_bios {
            todo!("select a dmg bios file");
        } else {
            todo!("remove the dmg bios file");
        }
    }
    if dmg_bios && ui.button("change dmg bios").clicked() {
        todo!("change the dmg bios file");
    }
    if ui.checkbox(&mut cgb_bios, "enable cgb bios").clicked() {
        if cgb_bios {
            todo!("select a cgb bios file");
        } else {
            todo!("remove the cgb bios file");
        }
    }
    if cgb_bios && ui.button("change cgb bios").clicked() {
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
