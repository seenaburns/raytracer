extern crate rand;
extern crate image;

mod ray;
mod vec3;
mod hitable;
mod camera;
mod material;

use vec3::{Vec3, random_in_unit_sphere};
use ray::Ray;
use hitable::*;
use camera::Camera;
use material::{Material, Lambertian, Metal};

use rand::{thread_rng, Rng};

use std::io::{self, Write};
use std::path::Path;

const NX: i32 = 200;
const NY: i32 = 100;
const NUM_SAMPLES: i32 = 100;
const MIN_DISTANCE: f64 = 0.000001;
const MAX_DISTANCE: f64 = 1000.0;
const DEPTH_MAX: i32 = 50;

fn color<T: Hitable>(r: &Ray, world: &HitableList<T>, depth: i32) -> Vec3 {
    match world.hit(r, MIN_DISTANCE, MAX_DISTANCE) {
        Some(h) => {
            if depth < 50 {
                match material::scatter(h.material, r, &h) {
                    Some((attentuation, scattered)) => {
                        attentuation * color(&scattered, world, depth+1)
                    }
                    None => Vec3::new(0.0,0.0,0.0)
                }
            } else {
                // Depth exceeded default color
                Vec3::new(0.0,0.0,0.0)
            }
        }
        None => {
            // Background
            let unit_dir = r.dir.normalized();
            let t = 0.5 * (unit_dir.y + 1.0);
            Vec3::new(1.0,1.0,1.0) * (1.0-t) + Vec3::new(0.5, 0.7, 1.0) * (t)
        }
    }
}

fn main() {
    io::stdout().write_fmt(format_args!("P3\n{} {}\n{}\n", NX, NY, 255)).unwrap();

    let camera = Camera {
        lower_left_corner: Vec3::new(-2.0, -1.0, -1.0),
        horizontal: Vec3::new(4.0, 0.0, 0.0),
        vertical: Vec3::new(0.0, 2.0, 0.0),
        origin: Vec3::new(0.0, 0.0, 0.0),
    };

    let m: Material = Material::Lambertian { m: Lambertian { albedo: Vec3::new(0.5,0.5,0.5) } };
    let world: HitableList<Sphere> = HitableList {
        items: vec![
            Sphere { center: Vec3::new(0.0,0.0,-1.0),    radius: 0.5,   material: m},
            Sphere { center: Vec3::new(0.0,-100.5,-1.0), radius: 100.0, material: m},
        ]
    };

    let mut rng = thread_rng();

    let mut outbuf: Vec<u8> = Vec::with_capacity((NX*NY*3) as usize);

    for j in (0..NY).rev() {
        for i in 0..NX {
            let mut c = Vec3::new(0.0, 0.0, 0.0);

            for _s in 0..NUM_SAMPLES {
                // Get percent offset from bottom left corner
                let u = (i as f64 + rng.next_f64()) / (NX as f64);
                let v = (j as f64 + rng.next_f64()) / (NY as f64);

                // Make ray
                let r = camera.get_ray(u, v);

                // Get color
                c += color(&r, &world, 0);
            }
            let c = c / (NUM_SAMPLES as f64);

            // Output for PPM
            // Gamma correct to 2: output color ^ (1/gamma) = x^(1/2) = sqrt
            let c = c.map(&|x: f64| x.sqrt());
            let c = c * 255.99;
            println!("{} {} {}", c.x as i32, c.y as i32, c.z as i32);

            // Save to buffer for image out
            outbuf.push(c.x as u8);
            outbuf.push(c.y as u8);
            outbuf.push(c.z as u8);
        }
    }

    image::save_buffer(&Path::new("out/out.png"), &outbuf, NX as u32, NY as u32, image::RGB(8)).unwrap();
}
