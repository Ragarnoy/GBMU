#[cfg(feature = "debug_render")]
use gb_lcd::render::RenderImage;
use gb_lcd::window::GBWindow;
#[cfg(feature = "debug_render")]
use gb_ppu::{
    SPRITE_LIST_RENDER_HEIGHT, SPRITE_LIST_RENDER_WIDTH, SPRITE_RENDER_HEIGHT, SPRITE_RENDER_WIDTH,
    TILEMAP_DIM, TILESHEET_HEIGHT, TILESHEET_WIDTH,
};

pub struct Windows {
    pub main: GBWindow,
    pub debug: Option<GBWindow>,
    pub input: Option<GBWindow>,
    #[cfg(feature = "debug_render")]
    pub tilemap: Option<(GBWindow, RenderImage<TILEMAP_DIM, TILEMAP_DIM>, bool)>,
    #[cfg(feature = "debug_render")]
    pub tilesheet: Option<(GBWindow, RenderImage<TILESHEET_WIDTH, TILESHEET_HEIGHT>)>,
    #[cfg(feature = "debug_render")]
    pub oam: Option<OAMConfig>,
}

#[cfg(feature = "debug_render")]
pub struct OAMConfig {
    pub window: GBWindow,
    pub viewport: RenderImage<SPRITE_RENDER_WIDTH, SPRITE_RENDER_HEIGHT>,
    pub list: RenderImage<SPRITE_LIST_RENDER_WIDTH, SPRITE_LIST_RENDER_HEIGHT>,
    pub display_list: bool,
    pub invert_color: bool,
}
