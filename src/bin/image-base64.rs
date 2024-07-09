extern crate image;

fn main() {
    let image: ImageBuffer<Rgba<u8>, Vec<u8>> = image::open("/home/dika/Pictures/Screenshots/1.png").unwrap().into();

    let mut buf: Vec<u8> = Vec::new();
    image
        .write_to(&mut Cursor::new(&mut buf), image::ImageFormat::Png)
        .expect("Couldn't write image to bytes.");
        
    let b64 = general_purpose::STANDARD.encode(buf);
    format!("{}", b64);
}
