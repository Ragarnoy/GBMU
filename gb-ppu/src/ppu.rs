use crate::memory::{Oam, Vram};
use crate::registers::Control;
use crate::{
    OBJECT_RENDER_HEIGHT, OBJECT_RENDER_WIDTH, TILEMAP_DIM, TILEMAP_TILE_COUNT, TILESHEET_HEIGHT,
    TILESHEET_TILE_COUNT, TILESHEET_WIDTH,
};
use gb_lcd::render::{RenderData, SCREEN_HEIGHT, SCREEN_WIDTH};

/// Pixel Process Unit: is in charge of selecting the pixel to be displayed on the lcd screen.
///
/// Memory field (Vram, OAM) and registers owned by the ppu are simply exposed by public function when required for examples for now.
/// This impl propably won't work once the cpu will need to access them.
pub struct PPU {
    vram: Vram,
    oam: Oam,
    control: Control,
    pixels: RenderData<SCREEN_WIDTH, SCREEN_HEIGHT>,
}

impl PPU {
    pub fn new() -> Self {
        Self {
            vram: Vram::new(),
            oam: Oam::new(),
            control: Control::new(),
            pixels: [[[255; 3]; SCREEN_WIDTH]; SCREEN_HEIGHT],
        }
    }

    pub fn pixels(&self) -> &RenderData<SCREEN_WIDTH, SCREEN_HEIGHT> {
        &self.pixels
    }

    pub fn control(&self) -> &Control {
        &self.control
    }

    pub fn control_mut(&mut self) -> &mut Control {
        &mut self.control
    }

    pub fn compute(&mut self) {
        for j in 0..SCREEN_HEIGHT {
            for i in 0..SCREEN_WIDTH {
                if i == 0 || i == SCREEN_WIDTH - 1 || j == 0 || j == SCREEN_HEIGHT - 1 {
                    self.pixels[j][i] = [255, 0, 0];
                } else if (i + j) % 2 == 0 {
                    self.pixels[j][i] = [0; 3];
                } else {
                    self.pixels[j][i] = [255; 3];
                }
            }
        }
    }

    pub fn overwrite_vram(&mut self, data: &[u8; Vram::SIZE as usize]) {
        self.vram.overwrite(data);
    }

    pub fn overwrite_oam(&mut self, data: &[u8; Oam::SIZE as usize]) {
        self.oam.overwrite(data);
    }

    /// Create an image of the current tilesheet.
    ///
    /// This function is used for debugging purpose.
    pub fn tilesheet_image(&self) -> RenderData<TILESHEET_WIDTH, TILESHEET_HEIGHT> {
        let mut image = [[[255; 3]; TILESHEET_WIDTH]; TILESHEET_HEIGHT];
        let mut x = 0;
        let mut y = 0;
        for k in 0..TILESHEET_TILE_COUNT {
            let tile = self.vram.read_8x8_tile(k).unwrap();
            for j in 0..8 {
                for i in 0..8 {
                    image[y * 8 + j][TILESHEET_WIDTH - (x + 1) * 8 + i] = match tile[j][i] {
                        3 => [0; 3],
                        2 => [85; 3],
                        1 => [170; 3],
                        0 => [255; 3],
                        _ => [255; 3],
                    }
                }
            }
            x += 1;
            if x * 8 >= TILESHEET_WIDTH {
                x = 0;
                y += 1;
            }
        }
        image
    }

    /// Create an image of the current tilemap.
    ///
    /// This function is used for debugging purpose.
    pub fn tilemap_image(&self, window: bool) -> RenderData<TILEMAP_DIM, TILEMAP_DIM> {
        let mut image = [[[255; 3]; TILEMAP_DIM]; TILEMAP_DIM];
        let mut x = 0;
        let mut y = 0;
        for k in 0..TILEMAP_TILE_COUNT {
            let index = self
                .vram
                .get_map_tile_index(
                    k,
                    if !window {
                        self.control.bg_tilemap_area()
                    } else {
                        self.control.win_tilemap_area()
                    },
                    self.control.bg_win_tiledata_area(),
                )
                .unwrap();
            let tile = self.vram.read_8x8_tile(index).unwrap();
            for j in 0..8 {
                for i in 0..8 {
                    image[y * 8 + j][TILEMAP_DIM - (x + 1) * 8 + i] = match tile[j][i] {
                        3 => [0; 3],
                        2 => [85; 3],
                        1 => [170; 3],
                        0 => [255; 3],
                        _ => [255; 3],
                    }
                }
            }
            x += 1;
            if x * 8 >= TILEMAP_DIM {
                x = 0;
                y += 1;
            }
        }
        image
    }

    /// Create an image of the currents objects.
    ///
    /// This function is used for debugging purpose.
    pub fn objects_image(&self) -> RenderData<OBJECT_RENDER_WIDTH, OBJECT_RENDER_HEIGHT> {
        let mut image = [[[255; 3]; OBJECT_RENDER_WIDTH]; OBJECT_RENDER_HEIGHT];
        let objects = self
            .oam
            .collect_all_objects()
            .expect("failed to collect objects for image");
        for object in objects {
            let x = object.x_pos().min(OBJECT_RENDER_WIDTH as u8 - 8) as usize;
            let y = object.y_pos().min(OBJECT_RENDER_HEIGHT as u8 - 16) as usize;
            let height = if self.control.obj_size() { 16 } else { 8 };
            for j in 0..height {
                let pixels_values = object
                    .get_pixels_row(j, &self.vram, self.control.obj_size())
                    .expect("invalid line passed");
                let y_img = y + j;
                for (i, pixel) in pixels_values.iter().enumerate() {
                    let x_img = OBJECT_RENDER_WIDTH - 8 - x + i;
                    match pixel {
                        3 => image[y_img][x_img] = [0; 3],
                        2 => image[y_img][x_img] = [85; 3],
                        1 => image[y_img][x_img] = [170; 3],
                        0 => {}
                        _ => {}
                    }
                }
            }
        }
        // draw screen outline
        for (y, column) in image.iter_mut().enumerate() {
            for (x, pixel) in column.iter_mut().enumerate() {
                if ((x == 7 || x == OBJECT_RENDER_WIDTH - 8)
                    && y >= 15
                    && y <= OBJECT_RENDER_HEIGHT - 16)
                    || ((y == 15 || y == OBJECT_RENDER_HEIGHT - 16)
                        && x >= 7
                        && x <= OBJECT_RENDER_WIDTH - 8)
                {
                    *pixel = [!pixel[0], !pixel[1], !pixel[2]];
                }
            }
        }
        image
    }
}

impl Default for PPU {
    fn default() -> PPU {
        PPU::new()
    }
}
