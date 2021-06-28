use crate::vec::*;
use crate::ray::*;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct HitRecord {
    pub p: Point,
    pub normal: Vec3,
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
            normal: Vec3::new(0.0,0.0,0.0),
            t: 0.0,
            front_face: false
        }
    }
}

pub trait Hittable {
    fn hit(self, r: &mut Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool; 
}
#[derive(Debug, Clone, PartialEq)]
pub struct HittablePool<T: Hittable> {
    pub objects: Vec<T>
}
impl<T: Hittable> HittablePool<T> {
    pub fn new() -> Self {
        HittablePool{
            objects: Vec::new()
        }
    }
    pub fn add(&mut self, object: T) {
        self.objects.push(object)
    }
}

impl<T: Hittable> Hittable for HittablePool<T>{
    fn hit(self, r: &mut Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool{
        let mut temporary_rec: HitRecord = HitRecord::default();
        let mut hitted = false;
        let mut closest = t_max;
        for object in self.objects {
            if object.hit(r,t_min, closest, &mut temporary_rec) {
                hitted = true;
                closest = temporary_rec.t;
                *rec = temporary_rec;
            }
        }
        hitted
    }
}