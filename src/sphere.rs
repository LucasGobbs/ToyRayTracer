use crate::vec::*;
use crate::ray::*;
use crate::hittable::*;
use crate::material::*;
use std::sync::Arc;
//#[derive(Clone)]
pub struct Sphere {
    pub center: Point,
    pub radius: f64,
    pub material: Material,
}
impl Sphere {
    pub fn new(x: f64, y: f64, z: f64, radius: f64, material: Material) -> Self {
        Sphere {
            center: Point::new(x,y,z),
            radius: radius,
            material: material,
        }
    }

}
impl Hittable for Sphere {
    fn hit(&self, r: &mut Ray, t_min: f64, t_max: f64, rec: HitRecord) -> (bool, HitRecord) {
        let mut new_rec = rec;
        let oc = r.origin - self.center;
        let a = r.dir.len_sqrt();
        let half_b = oc.dot(r.dir);
        let c = oc.len_sqrt() - self.radius*self.radius;
        let discriminant = half_b*half_b - a*c;
        if discriminant < 0.0 {
            return (false, new_rec)
        }

        let sqrtd = discriminant.sqrt();
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root{
                return (false, new_rec);
            }

        }
        new_rec.t = root;
        new_rec.material = Some(self.material);
        new_rec.p = r.at(new_rec.t);
        let outward_normal = (new_rec.p - self.center) / self.radius;
        new_rec.set_face_normal(*r, outward_normal);

        return (true, new_rec)
    } 
}