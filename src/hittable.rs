use crate::vec::*;
use crate::ray::*;
use crate::material::*;
use std::sync::Arc;
#[derive(Clone)]
pub struct HitRecord {
    pub p: Point,
    pub normal: Vec3,
    pub material: Option<Material>,
    pub t: f64,
    pub front_face: bool,
}
impl HitRecord {
    pub fn set_face_normal(&mut self, r: Ray, outward_normal: Vec3) {
        self.front_face = r.dir.dot(outward_normal) < 0.0;
        self.normal = if self.front_face {outward_normal} else {-outward_normal}; 
    }
    pub fn default() -> Self {
        HitRecord {
            p: Point::new(0.0,0.0,0.0),
            material: None,
            normal: Vec3::new(0.0,0.0,0.0),
            t: 0.0,
            front_face: false
        }
    }
}
//impl Copy for HitRecord { }
// impl Clone for HitRecord {
//     fn clone(&self) -> HitRecord {
//         HitRecord {
//             p: self.p,
//             material: self.material,
//             normal: self.normal,
//             t: self.t,
//             front_face: self.front_face
//         }
//     }
// }

pub trait Hittable {
    fn hit(&self, r: &mut Ray, t_min: f64, t_max: f64, rec: HitRecord) -> (bool, HitRecord); 
}

pub struct HittablePool{
    pub objects: Vec<Box<dyn Hittable + Sync>>
}

impl HittablePool {
    pub fn new() -> Self {
        Self {
            objects: Vec::new()
        }
    }

    pub fn add<T: Hittable + Sync + 'static>(&mut self, object: T)-> &mut Self {
        self.objects.push(Box::new(object));
        self
    }
}

impl Hittable for HittablePool{
    fn hit(&self, r: &mut Ray, t_min: f64, t_max: f64, rec: HitRecord) -> (bool, HitRecord){
        let mut temporary_rec: HitRecord = HitRecord::default();
        let mut hitted = false;
        let mut closest = t_max;
        let mut new_rec = rec;
        for object in self.objects.iter() {
            let (new_hitted, result_rec) = object.hit(r,t_min, closest, temporary_rec.clone());
            if new_hitted {
                hitted = true;
                closest = result_rec.t;
                new_rec = result_rec;
            }
        }
        (hitted, new_rec)
    }
}