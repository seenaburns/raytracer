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
    pub fn new(lookfrom: Vec3, lookat: Vec3, vup: Vec3, vfov: f64, aspect: f64) -> Camera {
        let theta = vfov * ::std::f64::consts::PI / 180.0;
        let half_height = (theta/2.0).tan();
        let half_width = aspect * half_height;

        // Define the basis for the camera
        let w = (lookfrom - lookat).normalized();
        let u = (vup.cross(w)).normalized();
        let v = w.cross(u);

        let origin = lookfrom;
        Camera {
            lower_left_corner: origin - half_width * u - half_height * v - w,
            horizontal: 2.0 * half_width * u,
            vertical: 2.0 * half_height * v,
            origin: origin,
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
