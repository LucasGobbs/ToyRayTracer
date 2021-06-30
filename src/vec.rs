use std::ops::*;
use rand::Rng;
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}
pub type Point = Vec3;
pub type Color = Vec3;

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
    pub fn dot(self, other: Vec3) -> f64 {
        self.x * other.x + 
        self.y * other.y + 
        self.z * other.z
    }
    pub fn cross(self, other: Vec3) -> Vec3 {
        return Vec3 {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }
    pub fn normal(self) -> Vec3 {
        let ln = self.len();
        Vec3 {
            x: self.x / ln,
            y: self.y / ln,
            z: self.z / ln
        }
    }
    pub fn normalize(&mut self)  {
        let ln = self.len();
        self.x /= ln;
        self.y /= ln;
        self.z /= ln;
    }
    pub fn random() -> Vec3 {
        let mut rng = rand::thread_rng();
        Vec3 {
            x: rng.gen_range(0.0 .. 1.0),
            y: rng.gen_range(0.0 .. 1.0),
            z: rng.gen_range(0.0 .. 1.0),
        }
    }
    pub fn random_range(min: f64, max: f64) -> Vec3 {
        let mut rng = rand::thread_rng();
        Vec3 {
            x: rng.gen_range(min .. max),
            y: rng.gen_range(min .. max),
            z: rng.gen_range(min .. max),
        }
    }
    
    pub fn random_in_unit_sphere() -> Vec3 {
        loop {
            let p = Vec3::random_range(-1.0,1.0);
            if p.len_sqrt() >= 1.0  {
                continue;
            }
            return p
        };
    }
    pub fn random_unit_vector() -> Vec3 {
        Vec3::random_in_unit_sphere().normal()
    }
    pub fn random_in_hemisphere(normal: Vec3) -> Vec3 {
        let in_unit = Vec3::random_in_unit_sphere();
        return if in_unit.dot(normal) > 0.0 {
            in_unit
        } else {
            -in_unit
        }
    }
    pub fn near_zero(self) -> bool {
        let s = 1e-8;
        return (self.x.abs() < s) && (self.y.abs() < s) && (self.z.abs() < s)
    }
    pub fn reflect(self, n: Vec3) -> Vec3 {
        self - n * self.dot(n) * 2.0
    }

    pub const RED: Vec3 = Vec3{x: 1.0, y: 0.0, z: 0.0};
    pub const LRED: Vec3 = Vec3{x: 0.8, y: 0.2, z: 0.2};

    pub const GREEN: Vec3 = Vec3{x: 0.0, y: 1.0, z: 0.0};
    pub const LGREEN: Vec3 = Vec3{x: 0.2, y: 0.8, z: 0.2};

    pub const BLUE: Vec3 = Vec3{x: 0.0, y: 0.0, z: 1.0};
    pub const LBLUE: Vec3 = Vec3{x: 0.2, y: 0.2, z: 0.8};

    pub const LYELLOW: Vec3 = Vec3{x: 0.8, y: 0.8, z: 0.0};
    pub const LPURPLE: Vec3 = Vec3{x: 0.8, y: 0.2, z: 0.8};

    pub const GRAY: Vec3 = Vec3{x: 0.5, y: 0.5, z: 0.5};
}


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
        Self {
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
impl Mul<f64> for Vec3 {
    type Output = Self;
    fn mul(self, value: f64) -> Self{
        return Self {
            x: self.x * value,
            y: self.y * value,
            z: self.z * value,
        }
    }
}
impl Mul<Vec3> for Vec3 {
    type Output = Self;
    fn mul(self, other: Vec3) -> Self{
        return Self {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
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
impl Div<f64> for Vec3 {
    type Output = Self;
    fn div(self, value: f64) -> Self{
        return Self {
            x: self.x / value,
            y: self.y / value,
            z: self.z / value,
        }
    }
}