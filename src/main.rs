use rfd::FileDialog;
#[cfg(feature = "debug_render")]
use sdl2::keyboard::Scancode;
use sdl2::{event::Event, keyboard::Keycode};

use gb_dbg::debugger::memory::MemoryEditorBuilder;
use gb_dbg::*;
use gb_lcd::{render, window::GBWindow};
use gb_ppu::PPU;

pub struct Memory {
    pub memory: Vec<u8>,
}

impl Default for Memory {
    fn default() -> Self {
        Self {
            memory: vec![0xFFu8; u16::MAX as usize],
        }
    }
}

impl dbg_interfaces::RW for Memory {
    fn read(&self, index: usize) -> u8 {
        *self.memory.get(index).unwrap()
    }

    fn write(&mut self, _index: usize, _value: u8) {
        self.memory[_index] = _value
    }
}

fn main() {
    let (sdl_context, video_subsystem, mut event_pump) =
        gb_lcd::init().expect("Error while initializing LCD");

    let bar_pixels_size = GBWindow::dots_to_pixels(&video_subsystem, render::MENU_BAR_SIZE)
        .expect("Error while computing bar size");
    let mut gb_window = GBWindow::new(
        "GBMU",
        (
            render::SCREEN_WIDTH as u32,
            render::SCREEN_HEIGHT as u32 + bar_pixels_size,
        ),
        true,
        &video_subsystem,
    )
    .expect("Error while building main window");
    let (width, height) = gb_window.sdl_window().size();

    gb_window
        .sdl_window_mut()
        .set_minimum_size(width, height)
        .expect("Failed to configure main window");

    let mut display = render::RenderImage::with_bar_size(bar_pixels_size as f32);
    let mut ppu = PPU::new();

    let mut debug_window = None;
    let mem = Memory::default();
    let _gbm_mem = MemoryEditorBuilder::new(mem)
        .with_address_range("VRam", 0..0xFF)
        .build();
    // Regression for now
    // let mut dbg_app = Debugger::new(gbm_mem, FlowController, Disassembler);

    #[cfg(feature = "debug_render")]
    let mut debug = false;

    'running: loop {
        gb_window
            .start_frame()
            .expect("Fail at the start for the main window");

        // render is updated just before drawing for now but we might want to change that later
        ppu.compute();
        display.update_render(ppu.pixels());
        // emulation render here
        display.draw();

        // set ui logic here
        egui::containers::TopBottomPanel::top("Top menu").show(gb_window.egui_ctx(), |ui| {
            egui::menu::bar(ui, |ui| {
                ui.set_height(render::MENU_BAR_SIZE);
                if ui.button("Load").clicked() {
                    let files = FileDialog::new()
                        .add_filter("rom", &["gb", "gbc", "rom"])
                        .set_directory(
                            std::env::current_dir()
                                .unwrap_or_else(|_| std::path::PathBuf::from("/")),
                        )
                        .pick_file();
                    println!("picked file: {:?}", files);
                }
                if ui.button("Debug").clicked() && debug_window.is_none() {
                    debug_window = Some(
                        GBWindow::new("GBMU Debug", (800, 600), false, &video_subsystem)
                            .expect("Error while building debug window"),
                    );
                }
            })
        });
        gb_window
            .end_frame()
            .expect("Fail at the end for the main window");

        if let Some(ref mut dgb_wind) = debug_window {
            dgb_wind
                .start_frame()
                .expect("Fail at the start for the debug window");
            // dbg_app.draw(dgb_wind.egui_ctx());
            // Regression for now
            dgb_wind
                .end_frame()
                .expect("Fail at the end for the debug window");
        }

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    // here for debug, maybe remove later ?
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                #[cfg(feature = "debug_render")]
                sdl2::event::Event::KeyDown {
                    window_id,
                    scancode,
                    ..
                } => {
                    if gb_window.sdl_window().id() == window_id && scancode == Some(Scancode::Grave)
                    {
                        debug = !debug;
                        display.switch_draw_mode(debug);
                        gb_window.set_debug(debug);
                    }
                }
                Event::Window {
                    win_event,
                    window_id,
                    ..
                } => match win_event {
                    sdl2::event::WindowEvent::SizeChanged(width, height) => {
                        if gb_window.sdl_window().id() == window_id {
                            gb_window
                                .resize((width as u32, height as u32), &video_subsystem)
                                .expect("Fail to resize GB window");
                            display.resize(gb_window.sdl_window().size());
                        } else if let Some(ref mut dbg_wind) = debug_window {
                            if dbg_wind.sdl_window().id() == window_id {
                                dbg_wind
                                    .resize((width as u32, height as u32), &video_subsystem)
                                    .expect("Fail to resize debug window");
                            }
                        }
                    }
                    sdl2::event::WindowEvent::Close => {
                        if gb_window.sdl_window().id() == window_id {
                            break 'running;
                        } else if let Some(ref mut dbg_wind) = debug_window {
                            if dbg_wind.sdl_window().id() == window_id {
                                debug_window = None;
                            }
                        }
                    }
                    _ => {}
                },
                _ => {
                    if !gb_window.send_event(&event, &sdl_context) {
                        if let Some(ref mut dbg_wind) = debug_window {
                            dbg_wind.send_event(&event, &sdl_context);
                        }
                    }
                }
            }
        }
        // std::thread::sleep(::std::time::Duration::new(0, 1_000_000_000u32 / 60));
    }
}
