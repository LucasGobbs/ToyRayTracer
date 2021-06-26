use std::ops::*;
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}
impl Vec3{
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 {
            x: x,
            y: y,
            z: z,
        }
    }
    pub fn fill(value: f64) -> Vec3 {
        Vec3 {
            x: value,
            y: value,
            z: value,
        }
    }
    pub fn len(self) -> f64 {
        self.len_sqrt().sqrt()
    }
    pub fn len_sqrt(self) -> f64 {
        self.x * self.x + 
        self.y * self.y + 
        self.z * self.z
    }
}
// a + b
impl Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x, 
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}
impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        };
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x, 
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}
impl SubAssign for Vec3 {
    fn sub_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        };
    }
}
impl Neg for Vec3{
    type Output = Self;
    fn neg(self) -> Self {
        Vec3 {
            x:  -self.x,
            y:  -self.y,
            z: -self.z,
        }
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, value: f64){
        *self = Self {
            x: self.x * value,
            y: self.y * value,
            z: self.z * value,
        }
    }
}
impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, value: f64){
        *self = Self {
            x: self.x / value,
            y: self.y / value,
            z: self.z / value,
        }
    }
}