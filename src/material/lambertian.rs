use crate::vec::*;
use crate::ray::*;
use crate::hittable::*;
use crate::material::*;
#[derive(Clone, Copy)]
pub struct Lambertian{
    albedo: Color
}
impl Lambertian{
    pub fn new(albedo: Color) -> Lambertian {
        Lambertian {
            albedo: albedo,
        }
    }
}
impl Scatterable for Lambertian {
    fn scatter(self, r_in: Ray, rec: HitRecord)-> (bool ,Ray, Color){
        let mut scatter_direction = rec.normal + Vec3::random_unit_vector();

        if scatter_direction.near_zero(){
            scatter_direction = rec.normal;
        }
        let new_ray = Ray::new(rec.p,scatter_direction);
        let attenuation = self.albedo;

        (true, new_ray, attenuation)
    }
}