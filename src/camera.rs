use super::vec3::Vec3;
use super::ray::Ray;

pub struct Camera {
    pub lower_left_corner: Vec3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
    pub origin: Vec3,
}

impl Camera {
    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray {
            origin: self.origin,
            dir: self.lower_left_corner
                 + (self.horizontal * u)
                 + (self.vertical * v)
                 - self.origin,
        }
    }
}
