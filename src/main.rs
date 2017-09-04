extern crate image;
extern crate raytracer;

use raytracer::*;
use raytracer::vec3::{Vec3};
use raytracer::camera::Camera;
use raytracer::model::{hitable, Renderable, Model};
use raytracer::shader::texture;
use raytracer::shader::material::Material;

use std::io::Write;
use std::fs::File;
use std::path::Path;

const NX: i32 = 400;
const NY: i32 = 200;
const NUM_SAMPLES: i32 = 5;

fn main() {
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

    // Two Checker spheres
    // let checker = texture::checker_texture(
    //     texture::constant_texture(Vec3::new(0.2,0.3,0.1)),
    //     texture::constant_texture(Vec3::new(0.9,0.9,0.9)),
    //     10.0,
    // );
    let perlin = texture::perlin_noise_texture();
    let world: Box<Vec<Box<Renderable>>> = Box::new(vec![
        Box::new(Model::new(
            hitable::Sphere {
                center: Vec3::new(0.0,-1000.0,0.0),
                radius: 1000.0,
            },
            Material::lambertian(perlin.clone()),
        )),
        Box::new(Model::new(
            hitable::Sphere {
                center: Vec3::new(0.0,2.0,0.0),
                radius: 2.0,
            },
            Material::lambertian(perlin.clone()),
        )),
    ]);
    let lookfrom = Vec3::new(13.0,2.0,3.0);
    let lookat = Vec3::new(0.0,0.0,0.0);
    let camera = Camera::new(
        lookfrom,
        lookat,
        Vec3::new(0.0, 1.0, 0.0),
        20.0,
        (NX as f64) / (NY as f64),
        0.0,
        (lookfrom - lookat).length(),
    );

    // Random world
    // let world = render::random_scene();
    // let lookfrom = Vec3::new(16.0, 2.0, 4.0);
    // let lookat = Vec3::new(-3.0, 0.5, -1.0);
    // let camera = Camera::new(
    //     lookfrom,
    //     lookat,
    //     Vec3::new(0.0, 1.0, 0.0),
    //     15.0,
    //     (NX as f64) / (NY as f64),
    //     0.1,
    //     (lookfrom - lookat).length(),
    // );


    match std::env::args().nth(1) {
        Some(ref mode) if mode == "bench" => {
            bench::bench_rays_per_sec(30)
        }
        _ => {

            let (outbuf, runtime) = bench::time(|| {
                render::render(world, &camera, NX, NY, NUM_SAMPLES, true)
            });

            save_file(&outbuf, NX, NY, "out/out.ppm", Filetype::PPM);

            // Summary stats
            let rays = NX * NY * NUM_SAMPLES;
            writeln!(&mut ::std::io::stderr(), "{} rays in {} seconds, {} rays/sec", rays, runtime, rays as f64/runtime).unwrap();
        }
    }
}

// Supported output filetypes
enum Filetype {
    PPM,
    PNG
}

fn save_file(data: &Vec<i32>, nx: i32, ny: i32, output_path: &str, filetype: Filetype) {
    match filetype {
        Filetype::PPM => {
            // Can save with image, but makes binary
            // Keeping manual implementation for debugging
            let path = Path::new(output_path);
            let mut file = File::create(&path).unwrap();

            // Metadata
            file.write_fmt(format_args!("P3\n{} {}\n{}\n", NX, NY, 255)).unwrap();

            // Data, one rgb triple per line
            let mut i = 0;
            for x in data {
                file.write_fmt(format_args!("{} ", x)).unwrap();
                i += 1;
                if i % 3 == 0 {
                    file.write(b"\n").unwrap();
                }
            }
        }
        Filetype::PNG => {
            let outbuf_u8: Vec<u8> = data.iter().map(|&x| { x as u8 }).collect();
            image::save_buffer(&Path::new(output_path), &outbuf_u8, nx as u32, ny as u32, image::RGB(8)).unwrap();
        }
    }
}
