extern crate raytracer;
use raytracer::vec3::Vec3;

use std::io::{self, Write};

const NX: i32 = 200;
const NY: i32 = 100;

fn main() {
    io::stdout().write_fmt(format_args!("P3\n{} {}\n{}\n", NX, NY, 255)).unwrap();
    for j in (0..NY).rev() {
        for i in 0..NX {
            let color = Vec3 {
                x: (i as f64) / (NX as f64),
                y: (j as f64) / (NY as f64),
                z: 0.2f64
            };
            let color = color.mulScalar(255.99);
            println!("{} {} {}", color.x as i32, color.y as i32, color.z as i32);
        }
    }
}
