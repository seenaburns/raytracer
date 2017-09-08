use vec3::Vec3;
use ray::Ray;
use model::bvh::{AABB, BoundingBox};
use util::Axis;
use std::f64;
use std::f64::consts::PI;

#[derive(Debug, Clone, PartialEq)]
pub struct HitRecord {
    pub t: f64,
    pub p: Vec3,
    pub normal: Vec3,
    pub u: f64,
    pub v: f64,
}

// Hitable trait: function to check if a ray hits the object
pub trait Hitable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

impl Hitable for Vec<Box<Hitable>> {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut hit = None;
        let mut closest = t_max;
        for x in self {
            if let Some(h) = x.hit(r,t_min,closest) {
                closest = h.t;
                hit = Some(h)
            }
        }
        hit
    }
}

//
// Utilities
//
// Convenience functions
pub fn flip_normals<H: Hitable>(h: H) -> FlipNormals<H> {
    FlipNormals {
        h: h
    }
}

pub fn translate<H: Hitable>(h: H, offset: Vec3) -> Translate<H> {
    Translate {
        h: h,
        offset: offset
    }
}

pub fn rotate<H: Hitable + BoundingBox>(h: H, axis: Axis, degrees: f64) -> Rotate<H> {
    Rotate::new(h, axis, degrees)
}

// FlipNormals
pub struct FlipNormals<H: Hitable> {
    h: H,
}

impl<H: Hitable> Hitable for FlipNormals<H> {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        self.h.hit(r, t_min, t_max).map(|x| {
            HitRecord { normal: -x.normal, .. x }
        })
    }
}

impl<H: Hitable + BoundingBox> BoundingBox for FlipNormals<H> {
    fn bounding_box(&self) -> AABB {
        self.h.bounding_box()
    }
}

// Translate
pub struct Translate<H: Hitable> {
    h: H,
    offset: Vec3,
}

impl<H: Hitable> Hitable for Translate<H> {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let moved_r = Ray {
            origin: r.origin - self.offset,
            dir: r.dir,
        };

        self.h.hit(&moved_r, t_min, t_max).map(|x| {
            HitRecord { p: x.p + self.offset, .. x }
        })
    }
}

impl<H: Hitable + BoundingBox> BoundingBox for Translate<H> {
    fn bounding_box(&self) -> AABB {
        self.h.bounding_box()
    }
}

// Rotation
pub struct Rotate<H: Hitable + BoundingBox> {
    h: H,
    axis: Axis,
    cos_theta: f64,
    sin_theta: f64,
    bounding_box: AABB,
}

impl<H: Hitable + BoundingBox> Rotate<H> {
    pub fn new(h: H, axis: Axis, degrees: f64) -> Rotate<H> {
        let (cos_theta, sin_theta) = ::util::degrees_to_cos_and_sin(degrees);

        // Create bounding box
        let mut min = Vec3::new(f64::NEG_INFINITY,f64::NEG_INFINITY,f64::NEG_INFINITY);
        let mut max = Vec3::new(f64::INFINITY,f64::INFINITY,f64::INFINITY);

        let bbox = h.bounding_box();

        // Iterate over every point on bounding box, find rotated point position
        // Expand bounding box if needed
        for v in bbox.vertices() {
            let rot_v = v.rotate(&axis, cos_theta, sin_theta);

            for a in Axis::iterator() {
                if rot_v.get_axis(a) > max.get_axis(a) {
                    max = max.set_axis(a, rot_v.get_axis(a));
                }
                if rot_v.get_axis(a) < min.get_axis(a) {
                    min = min.set_axis(a, rot_v.get_axis(a));
                }
            }
        }

        Rotate {
            h: h,
            axis: axis,
            cos_theta: cos_theta,
            sin_theta: sin_theta,
            bounding_box: AABB {
                min: min,
                max: max,
            }
        }
    }
}

impl<H: Hitable + BoundingBox> Hitable for Rotate<H> {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        // Rotatel ray by -degrees
        let moved_r = Ray {
            origin: r.origin.rotate(&self.axis, self.cos_theta, -self.sin_theta),
            dir: r.dir.rotate(&self.axis, self.cos_theta, -self.sin_theta),
        };

        self.h.hit(&moved_r, t_min, t_max).map(|x| {
            HitRecord {
                p: x.p.rotate(&self.axis, self.cos_theta, self.sin_theta),
                normal: x.normal.rotate(&self.axis, self.cos_theta, self.sin_theta),
                .. x
            }
        })
    }
}

impl<H: Hitable + BoundingBox> BoundingBox for Rotate<H> {
    fn bounding_box(&self) -> AABB {
        self.bounding_box
    }
}
