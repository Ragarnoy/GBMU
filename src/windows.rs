use gb_lcd::{GBPixels, GBWindow};

#[derive(Debug, Clone)]
pub enum WindowType {
    Keybindings,
    Debugger,
}

pub struct Windows {
    pub main: GBPixels,
    pub debugger: Option<GBWindow>,
}

impl Windows {
    pub fn new(main: GBPixels) -> Self {
        Self {
            main,
            debugger: None,
        }
    }
}
