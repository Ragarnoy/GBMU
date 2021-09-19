use crate::memory::{Oam, PPUMem, Vram};
use crate::registers::{Control, PPURegisters, PalettesMono, Scrolling, Stat, WindowPos};
use crate::{
    OBJECT_LIST_PER_LINE, OBJECT_LIST_RENDER_HEIGHT, OBJECT_LIST_RENDER_WIDTH,
    OBJECT_RENDER_HEIGHT, OBJECT_RENDER_WIDTH, TILEMAP_DIM, TILEMAP_TILE_COUNT, TILESHEET_HEIGHT,
    TILESHEET_TILE_COUNT, TILESHEET_WIDTH,
};
use gb_lcd::render::{RenderData, SCREEN_HEIGHT, SCREEN_WIDTH};

use std::cell::RefCell;
use std::rc::Rc;

/// The Pixel Process Unit is in charge of selecting the pixel to be displayed on the lcd screen.
///
/// It owns the VRAM and the OAM, as well as a few registers.
///
/// Registers owned by the ppu are simply exposed by public function when required for examples for now.
/// This impl propably won't work once the cpu will need to access them.
pub struct PPU {
    vram: Rc<RefCell<Vram>>,
    oam: Rc<RefCell<Oam>>,
    control: Rc<RefCell<Control>>,
    stat: Rc<RefCell<Stat>>,
    scrolling: Rc<RefCell<Scrolling>>,
    dma: Rc<RefCell<u8>>,
    pal_mono: Rc<RefCell<PalettesMono>>,
    window_pos: Rc<RefCell<WindowPos>>,
    pixels: RenderData<SCREEN_WIDTH, SCREEN_HEIGHT>,
}

impl PPU {
    pub fn new() -> Self {
        Self {
            vram: Rc::new(RefCell::new(Vram::new())),
            oam: Rc::new(RefCell::new(Oam::new())),
            control: Rc::new(RefCell::new(Control::new())),
            stat: Rc::new(RefCell::new(Stat::new())),
            scrolling: Rc::new(RefCell::new(Scrolling::new())),
            dma: Rc::new(RefCell::new(0)),
            pal_mono: Rc::new(RefCell::new(PalettesMono::new())),
            window_pos: Rc::new(RefCell::new(WindowPos::new())),
            pixels: [[[255; 3]; SCREEN_WIDTH]; SCREEN_HEIGHT],
        }
    }

    /// Build and return a [PPUMem] struct to access/modify the memory of this ppu instance.
    pub fn memory(&self) -> PPUMem {
        PPUMem::new(Rc::clone(&self.vram), Rc::clone(&self.oam))
    }

    /// Build and return a [PPURegisters] struct to access/modify the registers of this ppu instance.
    pub fn registers(&self) -> PPURegisters {
        PPURegisters::new(
            Rc::clone(&self.control),
            Rc::clone(&self.stat),
            Rc::clone(&self.scrolling),
            Rc::clone(&self.dma),
            Rc::clone(&self.pal_mono),
            Rc::clone(&self.window_pos),
        )
    }

    pub fn pixels(&self) -> &RenderData<SCREEN_WIDTH, SCREEN_HEIGHT> {
        &self.pixels
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

    /// Create an image of the current tilesheet.
    ///
    /// This function is used for debugging purpose.
    pub fn tilesheet_image(&self) -> RenderData<TILESHEET_WIDTH, TILESHEET_HEIGHT> {
        let mut image = [[[255; 3]; TILESHEET_WIDTH]; TILESHEET_HEIGHT];
        let mut x = 0;
        let mut y = 0;
        let vram = self.vram.borrow();
        let palette = self.pal_mono.borrow();
        for k in 0..TILESHEET_TILE_COUNT {
            let tile = vram.read_8x8_tile(k).unwrap();
            for (j, row) in tile.iter().enumerate() {
                for (i, pixel) in row.iter().rev().enumerate() {
                    image[y * 8 + j][x * 8 + i] =
                        palette.bg().get_color(*pixel).unwrap_or_default().into();
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
        let vram = self.vram.borrow();
        let control = self.control.borrow();
        let palette = self.pal_mono.borrow();
        for k in 0..TILEMAP_TILE_COUNT {
            let index = vram
                .get_map_tile_index(
                    k,
                    if !window {
                        control.bg_tilemap_area()
                    } else {
                        control.win_tilemap_area()
                    },
                    control.bg_win_tiledata_area(),
                )
                .unwrap();
            let tile = vram.read_8x8_tile(index).unwrap();
            for (j, row) in tile.iter().enumerate() {
                for (i, pixel) in row.iter().rev().enumerate() {
                    image[y * 8 + j][x * 8 + i] =
                        palette.bg().get_color(*pixel).unwrap_or_default().into();
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

    /// Create an image of the currents objects placed relatively to the viewport.
    ///
    /// This function is used for debugging purpose.
    pub fn objects_image(&self) -> RenderData<OBJECT_RENDER_WIDTH, OBJECT_RENDER_HEIGHT> {
        let mut image = [[[255; 3]; OBJECT_RENDER_WIDTH]; OBJECT_RENDER_HEIGHT];
        let objects = self
            .oam
            .borrow()
            .collect_all_objects()
            .expect("failed to collect objects for image");
        let vram = self.vram.borrow();
        let control = self.control.borrow();
        let palette = self.pal_mono.borrow();
        let height = if control.obj_size() { 16 } else { 8 };
        for object in objects {
            let x = object.x_pos().min(OBJECT_RENDER_WIDTH as u8 - 8) as usize;
            let y = object.y_pos().min(OBJECT_RENDER_HEIGHT as u8 - 16) as usize;
            for j in 0..height {
                let pixels_values = object
                    .get_pixels_row(j, &vram, control.obj_size(), palette.obj())
                    .expect("invalid line passed");
                let y_img = y + j;
                for (i, (pixel_value, pixel_color)) in pixels_values.iter().rev().enumerate() {
                    if *pixel_value != 0 {
                        let x_img = x + i;
                        image[y_img][x_img] = (*pixel_color).into();
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

    /// Create an image of the currents objects.
    ///
    /// This function is used for debugging purpose.
    pub fn objects_list_image(
        &self,
    ) -> RenderData<OBJECT_LIST_RENDER_WIDTH, OBJECT_LIST_RENDER_HEIGHT> {
        let mut image = [[[255; 3]; OBJECT_LIST_RENDER_WIDTH]; OBJECT_LIST_RENDER_HEIGHT];
        let objects = self
            .oam
            .borrow()
            .collect_all_objects()
            .expect("failed to collect objects for image");
        let vram = self.vram.borrow();
        let control = self.control.borrow();
        let palette = self.pal_mono.borrow();
        let height = if control.obj_size() { 16 } else { 8 };
        for (r, object) in objects.iter().enumerate() {
            let x = (r % OBJECT_LIST_PER_LINE) * 8;
            let y = (r / OBJECT_LIST_PER_LINE) * 16;
            for j in 0..height {
                let pixels_values = object
                    .get_pixels_row(j, &vram, control.obj_size(), palette.obj())
                    .expect("invalid line passed");
                let y_img = y + j;
                for (i, (pixel_value, pixel_color)) in pixels_values.iter().rev().enumerate() {
                    if *pixel_value != 0 {
                        let x_img = x + i;
                        image[y_img][x_img] = (*pixel_color).into();
                    }
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

#[cfg(test)]
mod vram {
    use super::PPU;
    use crate::test_tools::TestAddress;
    use gb_bus::FileOperation;

    #[test]
    fn locking() {
        let ppu = PPU::new();
        let mut ppu_mem = ppu.memory();
        {
            let _lock = ppu.vram.borrow();
            ppu_mem
                .write(0x42, Box::new(TestAddress::root_vram()))
                .expect("Try write value into borrowed vram");
            let res = ppu_mem
                .read(Box::new(TestAddress::root_vram()))
                .expect("Try reading borrowed value from vram");
            assert_eq!(res, 0x00, "invalid value from borrowed vram");
        }
    }

    #[test]
    fn locking_mut() {
        let ppu = PPU::new();
        let mut ppu_mem = ppu.memory();
        {
            {
                let _lock = ppu.vram.borrow_mut();
                ppu_mem
                    .write(0x42, Box::new(TestAddress::root_vram()))
                    .expect("Try write value into borrowed vram");
                let res = ppu_mem
                    .read(Box::new(TestAddress::root_vram()))
                    .expect("Try reading mut borrowed value from vram");
                assert_eq!(res, 0xFF, "invalid value from vram");
            }
            let res = ppu_mem
                .read(Box::new(TestAddress::root_vram()))
                .expect("Try reading mut borrowed value from vram");
            assert_eq!(res, 0x00, "invalid value from vram");
        }
    }
}

#[cfg(test)]
mod oam {
    use super::PPU;
    use crate::test_tools::TestAddress;
    use gb_bus::FileOperation;

    #[test]
    fn locking() {
        let ppu = PPU::new();
        let mut ppu_mem = ppu.memory();
        {
            let _lock = ppu.oam.borrow();
            ppu_mem
                .write(0x42, Box::new(TestAddress::root_oam()))
                .expect("Try write value into borrowed oam");
            let res = ppu_mem
                .read(Box::new(TestAddress::root_oam()))
                .expect("Try reading borrowed value from oam");
            assert_eq!(res, 0x00, "invalid value from borrowed oam");
        }
    }

    #[test]
    fn locking_mut() {
        let ppu = PPU::new();
        let mut ppu_mem = ppu.memory();
        {
            {
                let _lock = ppu.oam.borrow_mut();
                ppu_mem
                    .write(0x42, Box::new(TestAddress::root_oam()))
                    .expect("Try write value into borrowed oam");
                let res = ppu_mem
                    .read(Box::new(TestAddress::root_oam()))
                    .expect("Try reading mut borrowed value from oam");
                assert_eq!(res, 0xFF, "invalid value from oam");
            }
            let res = ppu_mem
                .read(Box::new(TestAddress::root_oam()))
                .expect("Try reading mut borrowed value from oam");
            assert_eq!(res, 0x00, "invalid value from oam");
        }
    }
}
