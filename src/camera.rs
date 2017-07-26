use super::vec3::Vec3;
use super::ray::Ray;

const ASPECT_16_BY_9: f64 = 16.0/9.0;

pub struct Camera {
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    origin: Vec3,
}

impl Camera {
    pub fn new(vfov: f64, aspect: f64) -> Camera {
        let theta = vfov * ::std::f64::consts::PI / 180.0;
        let half_height = (theta/2.0).tan();
        let half_width = aspect * half_height;
        Camera {
            lower_left_corner: Vec3::new(-half_width, -half_height, -1.0),
            horizontal: Vec3::new(2.0*half_width, 0.0, 0.0),
            vertical: Vec3::new(0.0, 2.0*half_height, 0.0),
            origin: Vec3::new(0.0,0.0,0.0),
        }
    }

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
