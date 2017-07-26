extern crate rand;

use std::ops::*;
use rand::{Rand, Rng, random};

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

    pub fn map(self, f: &Fn(f64) -> f64) -> Vec3 {
        Vec3 { x: f(self.x), y: f(self.y), z: f(self.z), }
    }

    pub fn length(self) -> f64 {
        let len = self.squared_length();
        len.sqrt()
    }

    pub fn squared_length(self) -> f64 {
        self.x*self.x + self.y*self.y + self.z*self.z
    }

    pub fn normalized(self) -> Vec3 {
        self / self.length()
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

// Vector operations
impl Add for Vec3 {
    type Output = Vec3;
    fn add(self, other: Vec3) -> Vec3 {
        Vec3 { x: self.x + other.x, y: self.y + other.y, z: self.z + other.z, }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Vec3) {
        *self = Vec3 { x: self.x + other.x, y: self.y + other.y, z: self.z + other.z, };
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

impl Rand for Vec3 {
    fn rand<R: Rng>(rng: &mut R) -> Vec3 {
        Vec3::new(rng.gen(), rng.gen(), rng.gen())
    }
}

// Scalar operations
impl Add<f64> for Vec3 {
    type Output = Vec3;
    fn add(self, other: f64) -> Vec3 {
        self.map(&(|x: f64| x + other))
    }
}

impl Sub<f64> for Vec3 {
    type Output = Vec3;
    fn sub(self, other: f64) -> Vec3 {
        self.map(&(|x: f64| x - other))
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;
    fn mul(self, other: f64) -> Vec3 {
        self.map(&(|x: f64| x * other))
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;
    fn mul(self, other: Vec3) -> Vec3 {
        other * self
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;
    fn div(self, other: f64) -> Vec3 {
        self.map(&(|x: f64| x / other))
    }
}

// Other
pub fn random_in_unit_sphere() -> Vec3 {
    loop {
        let p = random::<Vec3>() * 2.0 - 1.0;
        if p.dot(p) <=  1.0 {
            return p
        }
    }
}

pub fn random_in_unit_disk() -> Vec3 {
    loop {
        let p = (random::<Vec3>() * 2.0 - 1.0) * Vec3::new(1.0,1.0,0.0);
        if p.dot(p) <=  1.0 {
            return p
        }
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_scalar() {
        assert!(Vec3::new(1.0,2.0,3.0) + 2.0 == Vec3::new(3.0,4.0,5.0))
    }

    #[test]
    fn test_sub_scalar() {
        assert!(Vec3::new(1.0,2.0,3.0) - 2.0 == Vec3::new(-1.0,0.0,1.0))
    }

    #[test]
    fn test_mul_scalar() {
        assert!(Vec3::new(1.0,2.0,3.0) * 2.0 == Vec3::new(2.0,4.0,6.0))
    }

    #[test]
    fn test_div_scalar() {
        assert!(Vec3::new(1.0,2.0,3.0) / 2.0 == Vec3::new(0.5,1.0,1.5))
    }

    #[test]
    fn test_length() {
        assert!(Vec3::new(-2.0,0.5,1.0).length() - 2.29129 < 0.00001)
    }

    #[test]
    fn test_squared_length() {
        assert!(Vec3::new(-2.0,1.0,1.0).squared_length() == 6.0)
    }

    #[test]
    fn test_normalized() {
        let v = Vec3::new(1.0,2.0,3.0);
        let l = 14.0f64;
        assert!(v.normalized() == v / l.sqrt())
    }

    #[test]
    fn test_normalized_unit() {
        let v = Vec3::new(1.0,0.0,0.0);
        assert!(v.normalized() == v)
    }

    #[test]
    fn test_dot() {
        let a = Vec3::new(1.0,2.0,3.0);
        let b = Vec3::new(4.0,5.0,6.0);
        assert!(a.dot(b) == 32.0)
    }

    #[test]
    fn test_cross() {
        let a = Vec3::new(1.0,2.0,3.0);
        let b = Vec3::new(4.0,5.0,6.0);
        assert!(a.cross(b) == Vec3::new(-3.0,6.0,-3.0))
    }

    #[test]
    fn test_reflect() {
        let v = Vec3::new(1.0,-1.0,0.0);
        let n = Vec3::new(0.0,1.0,0.0);
        assert!(reflect(v,n) == Vec3::new(1.0,1.0,0.0))
    }
}
