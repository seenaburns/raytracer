use std::ops::*;
use std::cmp::PartialEq;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { x: x, y: y, z: z, }
    }

    pub fn r(&self) -> &f64 {
        &self.x
    }

    pub fn g(&self) -> &f64 {
        &self.x
    }

    pub fn b(&self) -> &f64 {
        &self.x
    }

    pub fn map(self, f: &Fn(f64) -> f64) -> Vec3 {
        Vec3 { x: f(self.x), y: f(self.y), z: f(self.z), }
    }

    pub fn add_scalar(self, other: f64) -> Vec3 {
        self.map(&(|x: f64| x + other))
    }

    pub fn sub_scalar(self, other: f64) -> Vec3 {
        self.map(&(|x: f64| x - other))
    }

    pub fn mul_scalar(self, other: f64) -> Vec3 {
        self.map(&(|x: f64| x * other))
    }

    pub fn div_scalar(self, other: f64) -> Vec3 {
        self.map(&(|x: f64| x / other))
    }

    pub fn length(self) -> f64 {
        let len = self.squared_length();
        len.sqrt()
    }

    pub fn squared_length(self) -> f64 {
        self.x*self.x + self.y*self.y * self.z*self.z
    }

    pub fn normalized(self) -> Vec3 {
        self.div_scalar(self.length())
    }

    pub fn dot(self, other: Vec3) -> f64 {
        self.x*other.x + self.y*other.y + self.z*other.z
    }

    pub fn cross(self, other: Vec3) -> Vec3 {
        Vec3 {
            x:  (self.y * other.z - self.z * other.y),
            y: -(self.x * other.z - self.z * other.x),
            z:  (self.x * other.y - self.y * other.x),
        }
    }
}

impl Add for Vec3 {
    type Output = Vec3;
    fn add(self, other: Vec3) -> Vec3 {
        Vec3 { x: self.x + other.x, y: self.y + other.y, z: self.z + other.z, }
    }
}

impl Sub for Vec3 {
    type Output = Vec3;
    fn sub(self, other: Vec3) -> Vec3 {
        Vec3 { x: self.x - other.x, y: self.y - other.y, z: self.z - other.z, }
    }
}

impl Mul for Vec3 {
    type Output = Vec3;
    fn mul(self, other: Vec3) -> Vec3 {
        Vec3 { x: self.x * other.x, y: self.y * other.y, z: self.z * other.z, }
    }
}

impl Div for Vec3 {
    type Output = Vec3;
    fn div(self, other: Vec3) -> Vec3 {
        Vec3 { x: self.x - other.x, y: self.y - other.y, z: self.z - other.z, }
    }
}

impl Neg for Vec3 {
    type Output = Vec3;
    fn neg(self) -> Vec3 {
        Vec3 { x: -self.x, y: -self.y, z: -self.z, }
    }
}
