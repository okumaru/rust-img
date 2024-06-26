extern crate image;

fn main() {
    let img = image::open("/home/dika/Pictures/Screenshots/1.png").unwrap();

    match img.save("test.png") {
        Err(e) => eprintln!("Error writing file: {}", e),
        Ok(()) => println!("Done."),
    };
}
