#![allow(unused_variables)]
extern crate image;

mod hittable;
use hittable::*;
mod sphere;
use sphere::*;
mod ray;
use ray::*;

mod vec;
use vec::*;

use image::{ImageBuffer, RgbImage};
fn main() {
    let aspect = 16.0 / 9.0;
    let width = 512;
    let height = (width as f32 / aspect) as u32;
    let mut img: RgbImage = ImageBuffer::new(width, height );

    let viewport_height: f64 = 2.0;
    let viewport_width: f64 = aspect as f64 * viewport_height;
    let focal_length = 1.0;

    let origin = Point::new(0.0,0.0,0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new (0.0, viewport_height, 0.0);
    let lower_left_corner = 
        origin - 
        horizontal / 2.0 - 
        vertical / 2.0 - 
        Vec3::new(0.0,0.0,focal_length);

    let mut world: HittablePool<Sphere> = HittablePool::new();
    world.add(Sphere::new(Point::new(0.0,0.0,-1.0), 0.5));
    world.add(Sphere::new(Point::new(0.0,-100.5,-1.0), 100.0));

    for (x,y,pixel) in img.enumerate_pixels_mut(){
        let u = x as f64 / (width - 1) as f64;
        let v = y as f64 / (height - 1) as f64;

        let mut ray = Ray::new(origin, lower_left_corner + horizontal*u + vertical * v - origin);
        let color = ray_color(world.clone(), &mut ray);
        // println!("{} {} ",color.x,color.x as u8);
        *pixel = image::Rgb(color_to_pix(color));
    }
    img.save("Image.png").unwrap();
}

fn  ray_color<T: Hittable>(world: HittablePool<T>, r: &mut Ray) -> Color {
    let mut rec: HitRecord = HitRecord::default();
    
    if world.hit(r,0.0,f64::INFINITY, &mut rec) {
        return (rec.normal + Color::new(1.0,1.0,1.0)) * 0.5
    }

    let unit_dir = r.dir.normal();
    let t = 0.5 * (unit_dir.y + 1.0);
    Color::new(1.0,1.0,1.0) * t + Color::new(0.5,0.7,1.0) * (1.0_f64 - t) 
}


fn color_to_pix(color: Color) ->[u8; 3] {
    [(color.x * 255.0) as u8, (color.y * 255.0) as u8, (color.z * 255.0) as u8]
}