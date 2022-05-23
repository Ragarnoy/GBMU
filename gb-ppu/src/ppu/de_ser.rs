pub mod pixel_buffer {
    use super::super::Vram;
    use gb_lcd::render::{SCREEN_HEIGHT, SCREEN_WIDTH};
    use serde::{
        de::{Error, Expected, SeqAccess, Visitor},
        ser::SerializeSeq,
        Deserializer, Serializer,
    };
    use std::fmt;

    const PIXEL_BUFFER_SIZE: usize = SCREEN_HEIGHT * SCREEN_WIDTH * 3;

    type PixelsBuffer = [[[u8; 3]; SCREEN_WIDTH]; SCREEN_HEIGHT];

    pub fn serialize<S>(pixels: &PixelsBuffer, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(PIXEL_BUFFER_SIZE))?;
        for line in pixels.into_iter() {
            for pixel in line.into_iter() {
                for byte in pixel.into_iter() {
                    seq.serialize_element(byte)?;
                }
            }
        }
        seq.end()
    }

    struct ExpectedPixelBuffer;
    impl Expected for ExpectedPixelBuffer {
        fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(formatter, "a pixel buffer with {} bytes value", Vram::SIZE)
        }
    }

    struct DataVisitor;

    impl<'de> Visitor<'de> for DataVisitor {
        type Value = PixelsBuffer;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a pixel buffer")
        }

        fn visit_seq<S>(self, mut access: S) -> Result<PixelsBuffer, S::Error>
        where
            S: SeqAccess<'de>,
        {
            let mut pixels = [[[0; 3]; SCREEN_WIDTH]; SCREEN_HEIGHT];
            let mut b = 0;
            while let Some(byte) = access.next_element::<u8>()? {
                if b < PIXEL_BUFFER_SIZE {
                    let p = b / 3;
                    pixels[p / SCREEN_WIDTH][p % SCREEN_WIDTH][b % 3] = byte;
                }
                b += 1;
            }
            if b > PIXEL_BUFFER_SIZE {
                return Err(Error::invalid_length(b, &ExpectedPixelBuffer));
            }
            Ok(pixels)
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<PixelsBuffer, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_seq(DataVisitor {})
    }
}
