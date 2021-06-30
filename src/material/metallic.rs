use crate::vec::*;
use crate::ray::*;
use crate::hittable::*;
use crate::material::*;
#[derive(Clone, Copy)]
pub struct Metallic{
    albedo: Color,
    fuzz: f64,
}
impl Metallic{
    pub fn new(albedo: Color, fuzz: f64) -> Metallic {
        Metallic {
            albedo: albedo,
            fuzz: fuzz.clamp(0.0,1.0),
        }
    }

}
impl Scatterable for Metallic {
    fn scatter(self, r_in: Ray, rec: HitRecord)-> (bool ,Ray, Color){
        let reflected_direction = r_in.dir.normal().reflect(rec.normal);
        let scattered = Ray::new(rec.p, reflected_direction + Vec3::random_in_unit_sphere()*self.fuzz);
        
        let new_ray = Ray::new(rec.p,scattered.dir);
        let attenuation = self.albedo;

        (scattered.dir.dot(rec.normal) > 0.0, new_ray, attenuation)
    }
}
