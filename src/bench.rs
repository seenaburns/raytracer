use std::time::{Duration, Instant};

use vec3::Vec3;
use camera::Camera;

// Run and return seconds duration
pub fn run_bench<F>(f: F) -> f64
    where F: FnOnce() {
    let start = Instant::now();
    f();
    let end = Instant::now();

    let runtime = end.duration_since(start);
    let runtime_nanos = runtime.as_secs() * 1_000_000_000 + (runtime.subsec_nanos() as u64);
    runtime_nanos as f64 / 1_000_000_000.0
}

pub fn bench_scene(nx: i32, ny: i32, spp: i32) {
    let lookfrom = Vec3::new(16.0, 2.0, 4.0);
    let lookat = Vec3::new(-3.0, 0.5, -1.0);
    let camera = Camera::new(
        lookfrom,
        lookat,
        Vec3::new(0.0, 1.0, 0.0),
        15.0,
        (nx as f64) / (ny as f64),
        0.1,
        (lookfrom - lookat).length(),
    );

    let scene = ::render::random_scene();

    ::render::render(&scene, &camera, nx, ny, spp);
}

