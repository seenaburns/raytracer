use std::io::{self, Write};

const NX: i32 = 200;
const NY: i32 = 100;

fn main() {
    io::stdout().write_fmt(format_args!("P3\n{} {}\n{}\n", NX, NY, 255)).unwrap();
    for j in (0..NY).rev() {
        for i in (0..NX) {
            let r = (i as f32) / (NX as f32);
            let g = (j as f32) / (NY as f32);
            let b = 0.2f32;
            let ir = (255.99 * r) as i32;
            let ig = (255.99 * g) as i32;
            let ib = (255.99 * b) as i32;
            println!("{} {} {}", ir, ig, ib);
        }
    }
}
