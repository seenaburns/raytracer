extern crate raytracer;
use raytracer::vec3::Vec3;
use raytracer::ray::Ray;

use std::io::{self, Write};

const NX: i32 = 200;
const NY: i32 = 100;

fn hit_sphere(center: Vec3, radius: f64, ray: &Ray) -> f64 {
    let oc: Vec3 = ray.origin - center;
    let a = ray.dir.dot(ray.dir);
    let b = 2.0 * oc.dot(ray.dir);
    let c = oc.dot(oc) - radius * radius;
    let discriminant = b*b - 4.0*a*c;
    if (discriminant < 0.0) {
        -1.0
    } else {
        (-b - discriminant.sqrt()) / (2.0 * a)
    }
}

fn color(r: &Ray) -> Vec3 {
    let sphere = Vec3::new(0.0,0.0,-1.0);
    let t = hit_sphere(sphere, 0.5, r);
    if (t > 0.0) {
        // Sphere color
        let n = (r.point_at_parameter(t) - sphere).normalized();
        return Vec3::new(n.x+1.0, n.y+1.0, n.z+1.0).mul_scalar(0.5);;
    } else {
        // Background
        let unit_dir = r.dir.normalized();
        let t = 0.5 * (unit_dir.y + 1.0);
        Vec3::new(1.0,1.0,1.0).mul_scalar(1.0-t) + Vec3::new(0.5, 0.7, 1.0).mul_scalar(t)
    }

}

fn float_eq(a: f64, b: f64) -> bool {
    a - b < 0.0001
}

fn main() {
    io::stdout().write_fmt(format_args!("P3\n{} {}\n{}\n", NX, NY, 255)).unwrap();

    let lower_left_corner = Vec3::new(-2.0, -1.0, -1.0);
    let horizontal = Vec3::new(4.0, 0.0, 0.0);
    let vertical = Vec3::new(0.0, 2.0, 0.0);
    let origin = Vec3::new(0.0, 0.0, 0.0);

    for j in (0..NY).rev() {
        for i in 0..NX {
            let u = (i as f64) / (NX as f64);
            let v = (j as f64) / (NY as f64);

            let r = Ray {
                origin: origin,
                dir: lower_left_corner + horizontal.mul_scalar(u) + vertical.mul_scalar(v),
            };
            let c = color(&r);
            let c = c.mul_scalar(255.99);
            println!("{} {} {}", c.x as i32, c.y as i32, c.z as i32);
        }
    }
}
