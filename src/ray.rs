use crate::vec::*;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Ray {
    pub origin: Point,
    pub dir: Vec3,
}
impl Ray {
    pub fn new(origin: Point, dir: Vec3) -> Self {
        Ray {
            origin: origin,
            dir: dir
        }
    }
    pub fn at(self, t: f64) -> Point {
        self.origin + self.dir * t  
    }
}