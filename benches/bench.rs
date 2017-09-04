#![feature(test)]

extern crate test;
extern crate raytracer;
extern crate rand;

use raytracer::model::hitable::*;
use raytracer::model::Model;
use raytracer::model::Renderable;
use raytracer::shader::material::*;
use raytracer::vec3::Vec3;
use raytracer::ray::Ray;
use raytracer::model::bvh::{AABB, Node};
use rand::random;
use rand::distributions::IndependentSample;

const r_hit: Ray = Ray {
    origin: Vec3 { x: 0.0, y: 0.0, z: -1.0 },
    dir: Vec3 { x: 0.0, y: 0.0, z: 1.0 },
};
const r_miss: Ray = Ray {
    origin: Vec3 { x: 0.0, y: 0.0, z: -1.0 },
    dir: Vec3 { x: 10.0, y: 10.0, z: 1.0 },
};




#[bench]
fn bench_sphere_hit(b: &mut test::Bencher) {
    b.iter(|| {
        let res = Sphere::unit_sphere().hit(&r_hit, 0.0000001, 10000.0);
    });
}

#[bench]
fn bench_sphere_miss(b: &mut test::Bencher) {
    b.iter(|| {
        let res = Sphere::unit_sphere().hit(&r_miss, 0.0000001, 10000.0);
    });
}

#[bench]
fn bench_aabb_hit(b: &mut test::Bencher) {
    b.iter(|| {
        let res = AABB::unit_aabb().hit(&r_hit, 0.0000001, 10000.0);
    });
}

#[bench]
fn bench_aabb_miss(b: &mut test::Bencher) {
    b.iter(|| {
        let res = AABB::unit_aabb().hit(&r_miss, 0.0000001, 10000.0);
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
        let res = unit_bvh.hit(&r_hit, 0.0000001, 10000.0);
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
        let res = unit_bvh.hit(&r_miss, 0.0000001, 10000.0);
    });
}
