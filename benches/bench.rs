#![feature(test)]

extern crate test;
extern crate raytracer;
extern crate rand;

use raytracer::model::hitable::*;
use raytracer::model::Model;
use raytracer::model::Renderable;
use raytracer::model::sphere::Sphere;
use raytracer::model::cube::Cube;
use raytracer::model::rect::Rect;
use raytracer::shader::material::*;
use raytracer::vec3::Vec3;
use raytracer::ray::Ray;
use raytracer::model::bvh::{AABB, Node};
use raytracer::util::Axis;

const R_HIT: Ray = Ray {
    origin: Vec3 { x: 0.0, y: 0.0, z: -2.0 },
    dir: Vec3 { x: 0.0, y: 0.0, z: 1.0 },
};
const R_MISS: Ray = Ray {
    origin: Vec3 { x: 0.0, y: 0.0, z: -2.0 },
    dir: Vec3 { x: 10.0, y: 10.0, z: 1.0 },
};

#[bench]
fn bench_sphere_hit(b: &mut test::Bencher) {
    b.iter(|| {
        Sphere::unit_sphere().hit(&R_HIT, 0.0000001, 10000.0);
    });
}

#[bench]
fn bench_sphere_miss(b: &mut test::Bencher) {
    b.iter(|| {
        Sphere::unit_sphere().hit(&R_MISS, 0.0000001, 10000.0);
    });
}

#[bench]
fn bench_aabb_hit(b: &mut test::Bencher) {
    b.iter(|| {
        AABB::unit_aabb().hit(&R_HIT, 0.0000001, 10000.0);
    });
}

#[bench]
fn bench_aabb_miss(b: &mut test::Bencher) {
    b.iter(|| {
        AABB::unit_aabb().hit(&R_MISS, 0.0000001, 10000.0);
    });
}

#[bench]
fn bench_bvh_hit(b: &mut test::Bencher) {
    let unit_bvh = Node::new(
        vec![Box::new(Model::new(
            Sphere::unit_sphere(),
            Material::lambertian_constant(Vec3::new(0.5,0.5,0.5))
        ))]
    );
    b.iter(|| {
        unit_bvh.hit(&R_HIT, 0.0000001, 10000.0);
    });
}

#[bench]
fn bench_bvh_miss(b: &mut test::Bencher) {
    let unit_bvh = Node::new(
        vec![Box::new(Model::new(
            Sphere::unit_sphere(),
            Material::lambertian_constant(Vec3::new(0.5,0.5,0.5))
        ))]
    );
    b.iter(|| {
        unit_bvh.hit(&R_MISS, 0.0000001, 10000.0);
    });
}

#[bench]
fn bench_cube_hit(b: &mut test::Bencher) {
    b.iter(|| {
        Cube::unit_cube().hit(&R_HIT, 0.0000001, 10000.0);
    });
}

#[bench]
fn bench_cube_miss(b: &mut test::Bencher) {
    b.iter(|| {
        Cube::unit_cube().hit(&R_MISS, 0.0000001, 10000.0);
    });
}

#[bench]
fn bench_rect_hit(b: &mut test::Bencher) {
    let r = Rect::xy_rect(-1.0,1.0,-1.0,1.0,0.0);
    b.iter(|| {
        r.hit(&R_HIT, 0.0000001, 10000.0);
    });
}

#[bench]
fn bench_rect_miss(b: &mut test::Bencher) {
    let r = Rect::xy_rect(-1.0,1.0,-1.0,1.0,0.0);
    b.iter(|| {
        r.hit(&R_MISS, 0.0000001, 10000.0);
    });
}

#[bench]
fn bench_rect_flip_hit(b: &mut test::Bencher) {
    let r = flip_normals(Rect::xy_rect(-1.0,1.0,-1.0,1.0,0.0));
    b.iter(|| {
        r.hit(&R_HIT, 0.0000001, 10000.0);
    });
}

#[bench]
fn bench_rect_flip_miss(b: &mut test::Bencher) {
    let r = flip_normals(Rect::xy_rect(-1.0,1.0,-1.0,1.0,0.0));
    b.iter(|| {
        r.hit(&R_MISS, 0.0000001, 10000.0);
    });
}

#[bench]
fn bench_sphere_translate_hit(b: &mut test::Bencher) {
    let s = translate(Cube::unit_cube(), Vec3::new(0.0,0.0,-1.0));
    b.iter(|| {
        s.hit(&R_HIT, 0.0000001, 10000.0);
    });
}

#[bench]
fn bench_sphere_translate_miss(b: &mut test::Bencher) {
    let s = translate(Cube::unit_cube(), Vec3::new(0.0,0.0,-1.0));
    b.iter(|| {
        s.hit(&R_MISS, 0.0000001, 10000.0);
    });
}

#[bench]
fn bench_sphere_rotate_hit(b: &mut test::Bencher) {
    let s = rotate(Cube::unit_cube(), Axis::Y, 45.0);
    b.iter(|| {
        s.hit(&R_HIT, 0.0000001, 10000.0);
    });
}

#[bench]
fn bench_sphere_rotate_miss(b: &mut test::Bencher) {
    let s = rotate(Cube::unit_cube(), Axis::Y, 45.0);
    b.iter(|| {
        s.hit(&R_MISS, 0.0000001, 10000.0);
    });
}
