extern crate image;
extern crate raytracer;

use raytracer::*;
use raytracer::vec3::{Vec3};
use raytracer::camera::Camera;
use raytracer::model::{hitable, Renderable, Model};
use raytracer::model::hitable::{flip_normals, translate, rotate};
use raytracer::model::cube::Cube;
use raytracer::model::rect::Rect;
use raytracer::model::sphere::Sphere;
use raytracer::model::constant_medium::ConstantMedium;
use raytracer::shader::texture;
use raytracer::shader::material::Material;
use raytracer::util::Axis;

use std::io::Write;
use std::fs::File;
use std::path::Path;
use std::sync::Arc;

const NX: i32 = 200;
const NY: i32 = 200;
const NUM_SAMPLES: i32 = 500;

fn main() {
    // TODO: Read scene data from file, not from code

    // Two Checker spheres
    // let checker = texture::checker_texture(
    //     texture::constant_texture(Vec3::new(0.2,0.3,0.1)),
    //     texture::constant_texture(Vec3::new(0.9,0.9,0.9)),
    //     10.0,
    // );
    // let perlin = texture::perlin_noise_texture(5.0, 7);
    // let image_texture = texture::image_texture("data/earthimage.jpg");
    // let world: Box<Vec<Box<Renderable>>> = Box::new(vec![
    //     Box::new(Model::new(
    //         hitable::Sphere {
    //             center: Vec3::new(0.0,-1000.0,0.0),
    //             radius: 1000.0,
    //         },
    //         Material::lambertian(perlin.clone()),
    //     )),
    //     Box::new(Model::new(
    //         hitable::Sphere {
    //             center: Vec3::new(0.0,2.0,0.0),
    //             radius: 2.0,
    //         },
    //         Material::lambertian(perlin.clone()),
    //     )),
    //     Box::new(Model::new(
    //         hitable::Sphere {
    //             center: Vec3::new(0.0,8.0,0.0),
    //             radius: 2.0,
    //         },
    //         Material::diffuse_light_constant(Vec3::new(4.0,4.0,4.0)),
    //     )),
    //     Box::new(Model::new(
    //         Rect::xy_rect(
    //              3.0,
    //              5.0,
    //              1.0,
    //              3.0,
    //              -2.0
    //         ),
    //         Material::diffuse_light_constant(Vec3::new(4.0,4.0,4.0)),
    //     )),
    // ]);
    // let lookfrom = Vec3::new(26.0,6.0,3.0);
    // let lookat = Vec3::new(0.0,2.0,0.0);
    // let camera = Camera::new(
    //     lookfrom,
    //     lookat,
    //     Vec3::new(0.0, 1.0, 0.0),
    //     20.0,
    //     (NX as f64) / (NY as f64),
    //     0.0,
    //     (lookfrom - lookat).length(),
    // );

    // Cornell Box
    let mat_red   = Material::lambertian_constant(Vec3::new(0.65,0.05,0.05));
    let mat_green = Material::lambertian_constant(Vec3::new(0.12,0.45,0.15));
    let mat_white = Material::lambertian_constant(Vec3::new(0.73,0.73,0.73));
    // let mat_light = Material::diffuse_light_constant(Vec3::new(15.0,15.0,15.0));
    let mat_light = Material::diffuse_light_constant(Vec3::new(5.0,5.0,5.0));
    let mat_iso   = Material::isotropic_constant(Vec3::new(1.0,1.0,1.0));
    let world: Box<Vec<Box<Renderable>>> = Box::new(vec![
        // Colored walls
        Box::new(Model::new(
            flip_normals(Rect::yz_rect(0.0,555.0,0.0,555.0,555.0)),
            mat_green.clone()
        )),
        Box::new(Model::new(
            Rect::yz_rect(0.0,555.0,0.0,555.0,0.0),
            mat_red.clone()
        )),
        // Light
        Box::new(Model::new(
            // Rect::xz_rect(213.0,343.0,227.0,332.0,554.0), // small light
            Rect::xz_rect(113.0,443.0,127.0,432.0,554.0), // big light
            mat_light.clone()
        )),
        // Ceiling
        Box::new(Model::new(
            flip_normals(Rect::xz_rect(0.0,555.0,0.0,555.0,555.0)),
            mat_white.clone()
        )),
        // Floor
        Box::new(Model::new(
            Rect::xz_rect(0.0,555.0,0.0,555.0,0.0),
            mat_white.clone()
        )),
        // Back wall
        Box::new(Model::new(
            flip_normals(Rect::xy_rect(0.0,555.0,0.0,555.0,555.0)),
            mat_white.clone()
        )),
        // Cubes
        Box::new(Model::new(
            translate(
                rotate(
                    Cube::new(Vec3::new(165.0,165.0,165.0)),
                    Axis::Y,
                    -18.0
                ),
                Vec3::new(130.0,0.0,65.0)
            ),
            mat_white.clone()
        )),
        Box::new(Model::new(
            translate(
                rotate(
                    Cube::new(Vec3::new(165.0,330.0,165.0)),
                    Axis::Y,
                    15.0
                ),
                Vec3::new(265.0,0.0,295.0)
            ),
            mat_white.clone()
        )),
    ]);
    let lookfrom = Vec3::new(275.0,275.0,-950.0);
    let lookat = Vec3::new(275.0,275.0,0.0);
    let camera = Camera::new(
        lookfrom,
        lookat,
        Vec3::new(0.0, 1.0, 0.0),
        35.0,
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
                render::render(world, Arc::new(camera), NX, NY, NUM_SAMPLES, 4, true)
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
