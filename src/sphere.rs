use crate::vec::*;
use crate::ray::*;
use crate::hittable::*;

#[derive(Debug, Clone, PartialEq)]
pub struct Sphere {
    pub center: Point,
    pub radius: f64
}
impl Sphere {
    pub fn new(center: Point, radius: f64) -> Self {
        Sphere {
            center: center,
            radius: radius
        }
    }
}
impl Hittable for Sphere {
    fn hit(&self, r: &mut Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool{
        let oc = r.origin - self.center;
        let a = r.dir.len_sqrt();
        let half_b = oc.dot(r.dir);
        let c = oc.len_sqrt() - self.radius*self.radius;
        let discriminant = half_b*half_b - a*c;
        if discriminant < 0.0 {
            return false
        }

        let sqrtd = discriminant.sqrt();
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root{
                return false;
            }

        }
        rec.t = root;
        rec.p = r.at(rec.t);
        let outward_normal = (rec.p - self.center) / self.radius;
        rec.set_face_normal(*r, outward_normal);

        return true
    } 
}