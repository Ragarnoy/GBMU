use gb_ppu::ImageRGB;

pub fn load_image_to_frame<const WIDTH: usize, const HEIGHT: usize>(
    ppu_image: ImageRGB<WIDTH, HEIGHT>,
    pixel_frame: &mut [u8],
) {
    for (i, pixel) in pixel_frame.chunks_exact_mut(4).enumerate() {
        let x = i % WIDTH;
        let y = i / HEIGHT;
        pixel[0] = ppu_image[y][x][0];
        pixel[1] = ppu_image[y][x][1];
        pixel[2] = ppu_image[y][x][2];
        // always the same alpha value
        pixel[3] = u8::MAX;
    }
}
