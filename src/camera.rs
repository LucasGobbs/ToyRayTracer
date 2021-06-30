use crate::vec::*;
use crate::ray::*;
pub struct Camera {
    pub origin: Point, 
    pub horizontal: Vec3,
    pub vertical: Vec3,
    pub lower_left_corner: Vec3
}
impl Camera {
    pub fn new() -> Self {
        let aspect = 16.0 / 9.0;
        let viewport_height: f64 = 2.0;
        let viewport_width: f64 = aspect as f64 * viewport_height;
        
        let focal_length = 2.0;
        let origin = Point::new(0.0,0.0,1.5);
        let horizontal = Vec3::new(viewport_width, 0.0,0.0);
        let vertical = Vec3::new(0.0,viewport_height,0.0);
        Self {
            origin: origin,
            horizontal: horizontal,
            vertical: vertical,
            lower_left_corner: origin  -  horizontal / 2.0  -  vertical / 2.0  - Vec3::new(0.0,0.0,focal_length)
        }
    }
    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(self.origin, self.lower_left_corner + self.horizontal*u + self.vertical * v - self.origin)
    }
}
