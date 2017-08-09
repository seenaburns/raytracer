#![allow(dead_code)]
use time::PreciseTime;

use vec3::Vec3;
use camera::Camera;

// Benchmarks sample scene
// Prints iteration execution time and average
pub fn bench_rays_per_sec(n: i32) {
    let mut runs: Vec<f64> = Vec::new();
    for _ in 0..n {
        let bench_params = (100, 50, 5);
        let (_, runtime) = time(|| {
            bench_scene(bench_params.0, bench_params.1, bench_params.2)
        });
        let rays = bench_params.0 * bench_params.1 * bench_params.2;
        let rays_per_sec = rays as f64 / runtime;
        println!("{} rays in {} sec, {:.2} rays/sec", rays, runtime, rays_per_sec);
        runs.push(rays_per_sec);
    }
    println!("Avg: {:.2} rays/sec from {} runs", runs.iter().sum::<f64>() / (runs.len() as f64), runs.len());

}

// Run function and return result with seconds duration
pub fn time<F, T>(f: F) -> (T, f64)
    where F: FnOnce() -> T {
    let start = PreciseTime::now();
    let res = f();
    let end = PreciseTime::now();

    let runtime_nanos = start.to(end).num_nanoseconds().expect("Benchmark iter took greater than 2^63 nanoseconds");
    let runtime_secs = runtime_nanos as f64 / 1_000_000_000.0;
    (res, runtime_secs)
}

fn bench_scene(nx: i32, ny: i32, spp: i32) {
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

