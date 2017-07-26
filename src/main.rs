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

use rand::{thread_rng, Rng, random};

use std::io::{self, Write};
use std::path::Path;

const NX: i32 = 400;
const NY: i32 = 200;
const NUM_SAMPLES: i32 = 150;
const MIN_DISTANCE: f64 = 0.000001;
const MAX_DISTANCE: f64 = 1000.0;
const DEPTH_MAX: i32 = 50;

const COLOR_BLUE:    Vec3 = Vec3 { x: 0.5, y: 0.7, z: 1.0 };
const COLOR_WHITE:   Vec3 = Vec3 { x: 1.0, y: 1.0, z: 1.0 };
const COLOR_DEFAULT: Vec3 = Vec3 { x: 0.0, y: 0.0, z: 0.0 };

fn color<T: Hitable>(r: &Ray, world: &HitableList<T>, depth: i32) -> Vec3 {
    match world.hit(r, MIN_DISTANCE, MAX_DISTANCE) {
        Some(h) => {
            if depth < 50 {
                match material::scatter(h.material, r, &h) {
                    Some((attentuation, scattered)) => {
                        attentuation * color(&scattered, world, depth+1)
                    }
                    // No scatter ray produced
                    None => COLOR_DEFAULT
                }
            } else {
                // Depth exceeded default color
                COLOR_DEFAULT
            }
        }
        None => {
            // Background
            let unit_dir = r.dir.normalized();
            let t = 0.5 * (unit_dir.y + 1.0);
            COLOR_WHITE * (1.0-t) + COLOR_BLUE * (t)
        }
    }
}

fn random_scene() -> HitableList<Sphere> {
    let mut items = Vec::new();

    // Ground
    items.push(Sphere {
        center: Vec3::new(0.0,-1000.0,0.0),
        radius: 1000.0,
        material: Material::lambertian(Vec3::new(0.5,0.5,0.5)),
    });

    // Big spheres
    items.push(Sphere {
        center: Vec3::new(-4.0,1.0,0.0),
        radius: 1.0,
        material: Material::lambertian(random::<Vec3>() * random::<Vec3>()),
    });
    items.push(Sphere {
        center: Vec3::new(4.0, 1.0, 0.0),
        radius: 1.0,
        material: Material::metal((random::<Vec3>() + 1.0 * 0.5), random::<f64>() * 0.5),
    });
    items.push(Sphere {
        center: Vec3::new(0.0,1.0,0.0),
        radius: 1.0,
        material: Material::dielectric(1.5),
    });

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random::<f64>();
            let center = Vec3::new(
                a as f64 + 0.9 * random::<f64>(),
                0.2,
                b as f64 + 0.9 * random::<f64>(),
            );
            if (center - Vec3::new(4.0,0.2,0.0)).length() > 0.9 {
                let m: Material;
                if choose_mat < 0.8 {
                    // diffuse
                    m = Material::lambertian(random::<Vec3>() * random::<Vec3>());
                } else if choose_mat < 0.95 {
                    // metal
                    m = Material::metal(
                        (random::<Vec3>() + 1.0) * 0.5,
                        0.5 * random::<f64>(),
                    );

                } else {
                    // glass
                    m = Material::dielectric(1.5);

                }
                items.push(Sphere {
                    center: center,
                    radius: 0.2,
                    material: m,
                });
            }
        }
    }

    HitableList {
        items: items
    }
}

fn main() {
    io::stdout().write_fmt(format_args!("P3\n{} {}\n{}\n", NX, NY, 255)).unwrap();

    let lookfrom = Vec3::new(16.0, 2.0, 4.0);
    let lookat = Vec3::new(-3.0, 0.5, -1.0);
    let camera = Camera::new(
        lookfrom,
        lookat,
        Vec3::new(0.0, 1.0, 0.0),
        15.0,
        (NX as f64) / (NY as f64),
        0.1,
        (lookfrom - lookat).length(),
    );

    // let m: Material = Material::lambertian(Vec3::new(0.5,0.5,0.5));
    //
    // let R = (::std::f64::consts::PI / 4.0).cos();
    // let world: HitableList<Sphere> = HitableList {
    //     items: vec![
    //         Sphere { center: Vec3::new(0.0,0.0,-1.0),     radius: 0.5,   material: Material::lambertian(Vec3::new(0.1,0.2,0.5))},
    //         Sphere { center: Vec3::new(0.0,-100.5,-1.0),  radius: 100.0, material: Material::lambertian(Vec3::new(0.8,0.8,0.0))}, // ground
    //         Sphere { center: Vec3::new(1.0,0.0,-1.0),     radius: 0.5,   material: Material::metal(Vec3::new(0.8,0.6,0.2), 0.0)},
    //         Sphere { center: Vec3::new(-1.0,0.0,-1.0),    radius: 0.5,   material: Material::dielectric(1.5)},
    //         Sphere { center: Vec3::new(-1.0,0.0,-1.0),    radius: -0.45, material: Material::dielectric(1.5)},
    //         // Sphere { center: Vec3::new(-R,0.0,-1.0),     radius: R,   material: Material::lambertian(Vec3::new(0.0, 0.0, 1.0))},
    //         // Sphere { center: Vec3::new( R,0.0,-1.0),     radius: R,   material: Material::lambertian(Vec3::new(1.0, 0.0, 0.0))},
    //     ]
    // };
    let world = random_scene();

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

        // Write percentage progress
        if j % (NY / 10) == 0 {
            writeln!(&mut std::io::stderr(), "{}/{}", NY-j, NY);
        }
    }

    image::save_buffer(&Path::new("out/out.png"), &outbuf, NX as u32, NY as u32, image::RGB(8)).unwrap();
}
