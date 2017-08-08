// Module for rendering scene

extern crate rand;

use hitable::*;
use vec3::{Vec3};
use camera::Camera;
use ray::Ray;
use material::Material;

use std::io::Write;
use rand::random;

const MIN_DISTANCE: f64 = 0.000001;
const MAX_DISTANCE: f64 = 1000.0;
const DEPTH_MAX: i32 = 50;

const COLOR_BLUE:    Vec3 = Vec3 { x: 0.5, y: 0.7, z: 1.0 };
const COLOR_WHITE:   Vec3 = Vec3 { x: 1.0, y: 1.0, z: 1.0 };
const COLOR_DEFAULT: Vec3 = Vec3 { x: 0.0, y: 0.0, z: 0.0 };


// Renders scene
//
// Return buffer of RGB triples as ints from top left to bottom right
// e.g. R G B R G B R G B ...
//
// Arguments:
//
// * `nx` - width of image
// * `ny` - height of image
// * `spp` - samples per pixel
pub fn render<T: Hitable>(
    scene: &HitableList<T>,
    camera: &Camera,
    nx: i32,
    ny: i32,
    spp: i32
) -> Vec<i32>  {
    let mut outbuf: Vec<i32> = Vec::with_capacity((nx*ny*3) as usize);

    for j in (0..ny).rev() {
        for i in 0..nx {
            let mut c = Vec3::new(0.0, 0.0, 0.0);

            for _s in 0..spp {
                // Get percent offset from bottom left corner
                let u = (i as f64 + random::<f64>()) / (nx as f64);
                let v = (j as f64 + random::<f64>()) / (ny as f64);

                // Make ray
                let r = camera.get_ray(u, v);

                // Get color
                c += color(&r, &scene, 0);
            }
            let c = c / (spp as f64);

            // Output for PPM
            // Gamma correct to 2: output color ^ (1/gamma) = x^(1/2) = sqrt
            let c = c.map(&|x: f64| x.sqrt());
            let c = c * 255.99;
            // println!("{} {} {}", c.x as i32, c.y as i32, c.z as i32);

            // Save to buffer for image out
            outbuf.push(c.x as i32);
            outbuf.push(c.y as i32);
            outbuf.push(c.z as i32);
        }

        // Write percentage progress
        if j % (ny / 10) == 0 {
            writeln!(&mut ::std::io::stderr(), "{}/{}", ny-j, ny);
        }
    }

    outbuf
}

fn color<T: Hitable>(r: &Ray, world: &HitableList<T>, depth: i32) -> Vec3 {
    match world.hit(r, MIN_DISTANCE, MAX_DISTANCE) {
        Some(h) => {
            if depth < DEPTH_MAX {
                match ::material::scatter(h.material, r, &h) {
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

pub fn random_scene() -> HitableList<Sphere> {
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
        material: Material::metal((random::<Vec3>() + 1.0) * 0.5, random::<f64>() * 0.3),
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