extern crate rand;

use vec3::{Vec3, random_in_unit_disk};
use ray::Ray;
use rand::*;

#[allow(unused)]
pub struct Camera {
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    origin: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    lens_radius: f64,
}

impl Camera {
    pub fn new(lookfrom: Vec3, lookat: Vec3, vup: Vec3, vfov: f64, aspect: f64, aperture: f64, focus_dist: f64) -> Camera {
        let lens_radius = aperture / 2.0;
        let theta = vfov * ::std::f64::consts::PI / 180.0;
        let half_height = (theta/2.0).tan();
        let half_width = aspect * half_height;

        // Define the basis for the camera
        let w = (lookfrom - lookat).normalized();
        let u = (vup.cross(w)).normalized();
        let v = w.cross(u);

        let origin = lookfrom;
        Camera {
            lower_left_corner: origin - half_width * u * focus_dist - half_height * v * focus_dist - w * focus_dist,
            horizontal: 2.0 * half_width * u * focus_dist,
            vertical: 2.0 * half_height * v * focus_dist,
            origin: origin,
            u: u,
            v: v,
            w: w,
            lens_radius: lens_radius,
        }
    }

    pub fn get_ray(&self, s: f64, t: f64, rng: &mut XorShiftRng) -> Ray {
        let rd = random_in_unit_disk(rng) * self.lens_radius;
        let offset = self.u * rd.x + self.v * rd.y;
        Ray {
            origin: self.origin + offset,
            dir: self.lower_left_corner
                 + (self.horizontal * s)
                 + (self.vertical * t)
                 - self.origin
                 - offset,
        }
    }
}
