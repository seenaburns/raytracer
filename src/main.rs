extern crate rand;
extern crate image;
extern crate time;

mod ray;
mod vec3;
mod hitable;
mod camera;
mod material;
mod render;

mod bench;

use vec3::{Vec3};
use camera::Camera;

use std::io::Write;
use std::fs::File;
use std::path::Path;

const NX: i32 = 400;
const NY: i32 = 200;
const NUM_SAMPLES: i32 = 5;

fn main() {
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
    let world = render::random_scene();

    let (outbuf, runtime) = bench::time(|| {
        render::render(&world, &camera, NX, NY, NUM_SAMPLES)
    });

    save_file(&outbuf, NX, NY, "out/out.ppm", Filetype::PPM);

    // Summary stats
    let rays = NX * NY * NUM_SAMPLES;
    writeln!(&mut ::std::io::stderr(), "{} rays in {} seconds, {} rays/sec", rays, runtime, rays as f64/runtime);
}

// Supported output filetypes
enum Filetype {
    PPM,
    PNG
}

fn save_file(data: &Vec<i32>, nx: i32, ny: i32, outputPath: &str, filetype: Filetype) {
    match filetype {
        Filetype::PPM => {
            // Can save with image, but makes binary
            // Keeping manual implementation for debugging
            let mut path = Path::new(outputPath);
            let mut file = File::create(&path).unwrap();

            // Metadata
            file.write_fmt(format_args!("P3\n{} {}\n{}\n", NX, NY, 255)).unwrap();

            // Data, one rgb triple per line
            let mut i = 0;
            for x in data {
                file.write_fmt(format_args!("{} ", x)).unwrap();
                i += 1;
                if i % 3 == 0 {
                    file.write(b"\n");
                }
            }
        }
        Filetype::PNG => {
            let outbuf_u8: Vec<u8> = data.iter().map(|&x| { x as u8 }).collect();
            image::save_buffer(&Path::new(outputPath), &outbuf_u8, nx as u32, ny as u32, image::RGB(8)).unwrap();
        }
    }
}
