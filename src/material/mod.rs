

use crate::vec::*;
use crate::ray::*;
use crate::hittable::*;

pub mod lambertian;
pub mod metallic;
use crate::lambertian::*;
use crate::metallic::*;
#[derive(Clone, Copy)]
pub enum Material {
    Lambertian(Lambertian),
    Metallic(Metallic),
}
impl Material {
    pub fn lambertian(albedo: Color) -> Material {
        return Material::Lambertian(Lambertian::new(albedo))
    }
    pub fn metallic(albedo: Color, fuzz: f64) -> Material {
        return Material::Metallic(Metallic::new(albedo, fuzz))
    }
}
pub trait Scatterable {
    //fn scatter(self, r_in: Ray, rec: HitRecord, attenuation: Color, scattered: Ray)-> (bool ,Ray, Color);
    fn scatter(self, r_in: Ray, rec: HitRecord)-> (bool ,Ray, Color);
}

impl Scatterable for Material {
    fn scatter(self, r_in: Ray, rec: HitRecord)-> (bool ,Ray, Color){
        match self {
            Material::Lambertian(ref inner) => inner.scatter(r_in, rec),
            Material::Metallic(ref inner) => inner.scatter(r_in, rec),
        }
    }
}



