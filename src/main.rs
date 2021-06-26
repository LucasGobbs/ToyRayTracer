extern crate image;
mod vec;
use image::{ImageBuffer, RgbImage};
fn main() {
    let mut img: RgbImage = ImageBuffer::new(512, 512);
    for (x,y,pixel) in img.enumerate_pixels_mut(){
        let r = (0.3 * x as f32) as u8;
        let b = (0.3 * y as f32) as u8;
        *pixel = image::Rgb([r, 100, b]);
    }
    img.save("Image.png").unwrap();
}
