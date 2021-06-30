#![allow(unused_variables)]
#![allow(dead_code)]
extern crate image;

mod hittable;
use hittable::*;

mod material;
use crate::material::*;

mod camera;
use camera::*;

mod sphere;
use sphere::*;

mod ray;
use ray::*;

mod vec;
use vec::*;

use image::{ImageBuffer, RgbImage};

use indicatif::{ProgressBar, ProgressStyle};
use rayon::prelude::*;

fn main() {
    let aspect = 16.0 / 9.0;
    let width = 512;
    let height = (width as f32 / aspect) as u32;
    let mut img: RgbImage = ImageBuffer::new(width, height);

    let samples = 50;
    let depth = 60;

    let camera = Camera::new();

    
    let mut world = HittablePool::new();
    world
        //Medium
        .add(Sphere::new(0.0,0.0,-1.0, 0.5, Material::lambertian(Color::LYELLOW)))
        .add(Sphere::new(-1.0,0.0,-1.0, 0.5, Material::metallic(Color::GRAY, 0.01)))
        .add(Sphere::new(1.0,0.0,-1.0, 0.5, Material::metallic(Color::GRAY, 0.8)))

        //Big
        .add(Sphere::new(0.0,100.5,-1.0, 100.0, Material::lambertian(Color::new(0.2,0.5,0.2))))

        .add(Sphere::new(0.0,0.4, 0.0, 0.1, Material::metallic(Color::GRAY,0.1)))
        .add(Sphere::new(0.2,0.4, 0.0, 0.1, Material::metallic(Color::GRAY, 0.2)))
        .add(Sphere::new(0.4,0.4, 0.0, 0.1, Material::metallic(Color::GRAY, 0.3)))
        .add(Sphere::new(0.6,0.4, 0.0, 0.1, Material::lambertian(Color::LPURPLE)))
        .add(Sphere::new(0.8,0.4, 0.0, 0.1, Material::lambertian(Color::LRED)));

    
    let pb = ProgressBar::new(height as u64);
    pb.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({eta})")
        .progress_chars("#>-"));
   
    pb.set_position(0);
    

    let image_data: Vec<Vec<[u8; 3]>> =  (0..height).into_par_iter().map(|y| {
        let row: Vec<[u8; 3]> =(0..width).into_par_iter().map(|x|{
            let mut base_color = Color::new(0.0,0.0,0.0);
             for i in 0..samples {
                let u = (x as f64 + rand::random::<f64>()) / (width - 1) as f64;
                let v = (y as f64 + rand::random::<f64>()) / (height - 1) as f64;
                let mut ray = camera.get_ray(u, v);
                base_color += ray_color(&world, &mut ray, depth);
             }
            color_to_pix(base_color, samples)
  
        }).collect();
        pb.inc(1);
        row
    }).collect();

    for (y, row) in image_data.iter().enumerate(){
        for (x, pixel) in row.iter().enumerate(){
            img.put_pixel(x as u32, y as u32, image::Rgb(pixel.clone()));
            //img.put_pixel(x, y, pixel);
        }
    }
    img.save("Image.png").unwrap();
}

fn  ray_color(world: &HittablePool, r: &mut Ray, depth: i32) -> Color {
    let mut rec: HitRecord = HitRecord::default();
    if depth < 0 {
        return Color::new(0.0,0.0,0.0);
    }
    let (hitted, new_rec) = world.hit(r,0.00001,f64::INFINITY, rec);
    if  hitted {

        //let (scattered_result, scattered, attenuation) = rec.material.unwrap()//.scatter(r.clone(), rec);
        let (scattered_result, mut scattered, attenuation) = new_rec.material.unwrap().scatter(r.clone(), new_rec);
        if scattered_result {
            return attenuation * ray_color(world, &mut scattered,depth-1);
        } else {
            return Color::new(0.0,0.0,0.0);
        }
        // let target = new_rec.p + new_rec.normal + Vec3::random_in_hemisphere(new_rec.normal);
        // let mut ray = Ray::new(new_rec.p, target - new_rec.p);
        // return ray_color(world, &mut ray, depth-1) * 0.5
    }

    let unit_dir = r.dir.normal();
    let t = 0.5 * (unit_dir.y + 1.0);
    Color::new(1.0,1.0,1.0) * (1.0_f64 - t) + Color::new(0.5,0.7,1.0) * t 
}


fn color_to_pix(color: Color, samples: i32) ->[u8; 3] {
    let scale = 1.0 / samples as f64;
    let r = (color.x * scale).sqrt();
    let g = (color.y * scale).sqrt();
    let b = (color.z * scale).sqrt();
    
    [(r.clamp(0.0,0.999) * 256.0) as u8, 
     (g.clamp(0.0,0.999) * 256.0) as u8, 
     (b.clamp(0.0,0.999) * 256.0) as u8]
}