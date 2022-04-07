use gb_lcd::GBPixels;

pub struct Windows {
    pub main: GBPixels,
}

impl Windows {
    pub fn new(main: GBPixels) -> Self {
        Self { main }
    }
}
