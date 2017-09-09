// Module for rendering scene

extern crate rand;

use camera::Camera;
use model::{bvh, Renderable, Model};
use model::sphere::Sphere;
use ray::Ray;
use shader::material::Material;
use vec3::{Vec3};

use std::io::Write;
use std::thread;
use std::sync::Arc;
use rand::random;

const MIN_DISTANCE: f64 = 0.000001;
const MAX_DISTANCE: f64 = 1000.0;
const DEPTH_MAX: i32 = 50;

#[allow(unused)]
const COLOR_BLUE:    Vec3 = Vec3 { x: 0.5, y: 0.7, z: 1.0 };
#[allow(unused)]
const COLOR_WHITE:   Vec3 = Vec3 { x: 1.0, y: 1.0, z: 1.0 };
#[allow(unused)]
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
pub fn render (
    scene: Box<Renderable + Sync>,
    camera: Arc<Camera>,
    nx: i32,
    ny: i32,
    spp: i32,
    nthreads: i32,
    debug: bool,
) -> Vec<i32>  {
    let mut threads = vec![];

    let shareable_scene: Arc<Box<Renderable>> = Arc::new(scene);

    // Each thread takes 1/nthreads of the image split into horizontal slices
    // rev because bottom of image first
    for nth in (0..nthreads).rev() {
        {
            let camera = camera.clone();
            let s = shareable_scene.clone();
            threads.push(thread::spawn(move || {
                // Get horizontal slice, final thread takes any extra due to int truncation
                let start = (ny/nthreads) * nth;
                let end = if nth == nthreads - 1 { ny } else { (ny/nthreads) * (nth+1) };
                let slice = end - start;

                let capacity = slice * nx * 3;
                let mut outbuf: Vec<i32> = Vec::with_capacity(capacity as usize);

                for j in (start..end).rev() {
                    for i in 0..nx {
                        let mut c = Vec3::new(0.0, 0.0, 0.0);

                        for _s in 0..spp {
                            c += sample(i,j,nx,ny,&(**s),&(*camera));
                        }
                        let c = c / (spp as f64);

                        // Output for PPM
                        // Gamma correct to 2: output color ^ (1/gamma) = x^(1/2) = sqrt
                        let c = c.map(&|x: f64| x.sqrt());
                        let c = c * 255.99;
                        let c = c.map(&|x: f64| if x > 256.99 { 255.99 } else { x });

                        // Save to buffer for image out
                        outbuf.push(c.x as i32);
                        outbuf.push(c.y as i32);
                        outbuf.push(c.z as i32);
                    }

                    // Write percentage progress
                    if debug && j % ::std::cmp::max(slice / 10, 1) == 0 {
                        writeln!(&mut ::std::io::stderr(), "{}: {}/{}", nth, slice-(j-start), slice).unwrap();
                    }
                }

                (outbuf, nth)
            }));
        }
    }

    // Join and create final buffer from results
    let mut outbuf: Vec<i32> = Vec::with_capacity((nx * ny * 3) as usize);

    for t in threads {
        let (mut res, nth) = t.join().unwrap(); // If thread fails, unrecoverable
        if debug {
            writeln!(&mut ::std::io::stderr(), "Thread {} done", nth);
        }
        outbuf.append(&mut res);
    }

    outbuf
}

fn sample(x: i32, y: i32, nx: i32, ny: i32, scene: &Renderable, camera: &Camera) -> Vec3 {
    // Get percent offset from bottom left corner
    let u = (x as f64 + random::<f64>()) / (nx as f64);
    let v = (y as f64 + random::<f64>()) / (ny as f64);

    // Make ray
    let r = camera.get_ray(u, v);

    // Get color
    color(&r, scene, 0)
}

fn color(r: &Ray, world: &Renderable, depth: i32) -> Vec3 {
    match world.hit(r, MIN_DISTANCE, MAX_DISTANCE) {
        Some((h, material)) => {
            let emitted = material.emitted(h.u, h.v, &h.p).unwrap_or(Vec3::new(0.0,0.0,0.0));
            if depth < DEPTH_MAX {
                match material.scatter(r, &h) {
                    Some((attentuation, scattered)) => {
                        emitted + attentuation * color(&scattered, world, depth+1)
                    }
                    // No scatter ray produced
                    None => emitted
                }
            } else {
                // Depth exceeded default color
                COLOR_DEFAULT
            }
        }
        None => {
            // Background

            // Sky
            // let unit_dir = r.dir.normalized();
            // let t = 0.5 * (unit_dir.y + 1.0);
            // COLOR_WHITE * (1.0-t) + COLOR_BLUE * (t)

            // Black
            COLOR_DEFAULT
        }
    }
}

pub fn random_scene() -> Box<Renderable + Sync> {
    let mut items: Vec<Box<bvh::BVHItem>> = Vec::new();

    // Ground
    items.push(Box::new(Model::new(
        Sphere {
            center: Vec3::new(0.0,-1000.0,0.0),
            radius: 1000.0
        },
        Material::lambertian_constant(random::<Vec3>()),
    )));

    // Big spheres
    items.push(Box::new(Model::new(
        Sphere {
            center: Vec3::new(-4.0,1.0,0.0),
            radius: 1.0,
        },
        Material::lambertian_constant(random::<Vec3>() * random::<Vec3>()),
    )));
    items.push(Box::new(Model::new(
        Sphere {
            center: Vec3::new(4.0, 1.0, 0.0),
            radius: 1.0,
        },
        Material::metal((random::<Vec3>() + 1.0) * 0.5, random::<f64>() * 0.3),
    )));
    items.push(Box::new(Model::new(
        Sphere {
            center: Vec3::new(0.0,1.0,0.0),
            radius: 1.0,
        },
        Material::dielectric(1.5),
    )));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random::<f64>();
            let center = Vec3::new(
                a as f64 + 0.9 * random::<f64>(),
                0.2,
                b as f64 + 0.9 * random::<f64>(),
            );
            if (center - Vec3::new(4.0,0.2,0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    // diffuse
                    items.push(Box::new(Model::new(
                        Sphere {
                            center: center,
                            radius: 0.2,
                        },
                        Material::lambertian_constant(random::<Vec3>() * random::<Vec3>()),
                    )));
                } else if choose_mat < 0.95 {
                    // metal
                    items.push(Box::new(Model::new(
                        Sphere {
                            center: center,
                            radius: 0.2,
                        },
                        Material::metal(
                            (random::<Vec3>() + 1.0) * 0.5,
                            0.5 * random::<f64>(),
                        ),
                    )));

                } else {
                    // glass
                    items.push(Box::new(Model::new(
                        Sphere {
                            center: center,
                            radius: 0.2,
                        },
                        Material::dielectric(1.5),
                    )));

                }
            }
        }
    }

    Box::new(bvh::Node::new(items))
}
