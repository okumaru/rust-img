use std::io::Cursor;

extern crate image;

use base64::{engine::general_purpose, Engine};
use image::{ImageBuffer, Rgba};

fn main() {
    let image: ImageBuffer<Rgba<u8>, Vec<u8>> = image::open("C:/Users/Dika/Documents/github/warehouse/assets/soh.png").unwrap().into();

    let mut buf: Vec<u8> = Vec::new();
    image
        .write_to(&mut Cursor::new(&mut buf), image::ImageFormat::Png)
        .expect("Couldn't write image to bytes.");

    let b64 = general_purpose::STANDARD.encode(buf);

    dbg!("{}", b64);
}
