// https://github.com/lopossumi/Rust-Output-Image

extern crate image;

use image::{ImageBuffer, Rgb, RgbImage};

fn main() {
    const IMAGE_WIDTH: u32 = 254;
    const IMAGE_HEIGHT: u32 = 254;

    let mut buffer: RgbImage = ImageBuffer::new(IMAGE_WIDTH, IMAGE_HEIGHT);

    for (x, y, pixel) in buffer.enumerate_pixels_mut() {
        let r = x as f64 / (IMAGE_WIDTH - 1) as f64;
        let g = y as f64 / (IMAGE_HEIGHT - 1) as f64;
        let b = 0.50;

        let ir = (255.999 * r) as u8;
        let ig = (255.999 * g) as u8;
        let ib = (255.999 * b) as u8;

        *pixel = Rgb([ir, ig, ib]);
    }

    match buffer.save("image.png") {
        Err(e) => eprintln!("Error writing file: {}", e),
        Ok(()) => println!("Done."),
    };
}
