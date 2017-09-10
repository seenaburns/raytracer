use vec3::Vec3;
use ray::Ray;
use model::bvh::{AABB, BoundingBox};
use model::hitable::{HitRecord, Hitable};
use model::hitable::flip_normals;
use model::rect::Rect;

pub struct Cube {
    min: Vec3,
    max: Vec3,
    sides: Vec<Box<Hitable>>
}

impl Cube {
    // Cube of size dimensions
    pub fn new(dimensions: Vec3) -> Cube {
        let p0 = Vec3::new(0.0,0.0,0.0);
        let p1 = dimensions;
        Cube::new_from_min_max(p0,p1)
    }

    pub fn new_from_min_max(p0: Vec3, p1: Vec3) -> Cube {
        let mut sides: Vec<Box<Hitable>> = Vec::with_capacity(6);

        sides.push(Box::new(Rect::xy_rect(p0.x, p1.x, p0.y, p1.y, p1.z)));
        sides.push(Box::new(flip_normals(Rect::xy_rect(p0.x, p1.x, p0.y, p1.y, p0.z))));
        sides.push(Box::new(Rect::xz_rect(p0.x, p1.x, p0.z, p1.z, p1.y)));
        sides.push(Box::new(flip_normals(Rect::xz_rect(p0.x, p1.x, p0.z, p1.z, p0.y))));
        sides.push(Box::new(Rect::yz_rect(p0.y, p1.y, p0.z, p1.z, p1.x)));
        sides.push(Box::new(flip_normals(Rect::yz_rect(p0.y, p1.y, p0.z, p1.z, p0.x))));

        Cube {
            min: p0,
            max: p1,
            sides: sides
        }
    }

    pub fn unit_cube() -> Cube {
        Cube::new_from_min_max(Vec3::new(-1.0,-1.0,-1.0), Vec3::new(1.0,1.0,1.0))
    }
}

impl Hitable for Cube {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        self.sides.hit(r,t_min,t_max)
    }
}

impl BoundingBox for Cube {
    fn bounding_box(&self) -> AABB {
        AABB {
            min: self.min,
            max: self.max,
        }
    }
}
