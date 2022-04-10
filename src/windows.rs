use gb_lcd::GBPixels;

#[derive(Debug, Clone)]
pub enum WindowType {
    Keybindings,
    // Debugger,
}

pub struct Windows {
    pub main: GBPixels,
}

impl Windows {
    pub fn new(main: GBPixels) -> Self {
        Self { main }
    }
}
