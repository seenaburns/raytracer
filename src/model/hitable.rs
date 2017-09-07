use vec3::Vec3;
use ray::Ray;
use model::bvh::{AABB, BoundingBox};
use util::Axis;
use std::f64::consts::PI;

#[derive(Debug, Clone, PartialEq)]
pub struct HitRecord {
    pub t: f64,
    pub p: Vec3,
    pub normal: Vec3,
    pub u: f64,
    pub v: f64,
}

// Hitable trait includes
// 1. function to check if a ray hits the object
// 2. defining a bounding box for the object
// Though conceptually separate all primitive objects currently define both. Once a hitable without
// a bounding box (e.g. infinite floor) is needed, this can be separated into two traits and use a
// combination as the trait object.
pub trait Hitable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}


#[derive(Debug)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
}

impl Sphere {
    pub fn hit_at_t(&self, r: &Ray, t: f64) -> HitRecord {
        let surface_hit = r.point_at_parameter(t);
        let (u, v) = Sphere::get_sphere_uv(&((surface_hit - self.center) / self.radius));
        HitRecord {
            t: t,
            p: surface_hit,
            normal: (surface_hit - self.center) / self.radius,
            u: u,
            v: v,
        }
    }

    pub fn unit_sphere() -> Sphere {
        Sphere {
            center: Vec3::new(0.0,0.0,0.0),
            radius: 1.0,
        }
    }

    fn get_sphere_uv(p: &Vec3) -> (f64, f64) {
        let phi = p.z.atan2(p.x); // angle around axis
        let theta = p.y.asin(); // angle from down the pole
        let u = 1.0 - (phi + PI) / (2.0 * PI); // remap (-PI, PI) to (0,1)
        let v = (theta + PI/2.0) / PI; // remap (-PI/2, PI/2) to (0,1)
        (u, v)
    }
}

impl Hitable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc: Vec3 = r.origin - self.center;
        let a = r.dir.dot(r.dir);
        let b = oc.dot(r.dir);
        let c = oc.dot(oc) - self.radius * self.radius;
        let discriminant = b*b - a*c;
        if discriminant > 0.0 {
            // 1 or 2 hits
            // Try each solution for t (tmp) to see if it is within range
            let tmp: f64 = (-b - (b*b - a*c).sqrt()) / a;
            if tmp < t_max && tmp > t_min {
                return Some(self.hit_at_t(&r, tmp))
            }
            let tmp: f64 = (-b + (b*b - a*c).sqrt()) / a;
            if tmp < t_max && tmp > t_min {
                return Some(self.hit_at_t(&r, tmp))
            }
        }
        None
    }
}

impl BoundingBox for Sphere {
    fn bounding_box(&self) -> AABB {
        AABB {
            min: self.center - self.radius,
            max: self.center + self.radius,
        }
    }
}

//
// XY_RECT
//

pub struct XYRect {
    pub x0: f64,
    pub x1: f64,
    pub y0: f64,
    pub y1: f64,
    pub k: f64,
}

impl Hitable for XYRect {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let t = (self.k - r.origin.z) / r.dir.z; // find where ray intersects z plane

        if (t < t_min || t > t_max) {
            return None;
        }

        let p = r.point_at_parameter(t);

        if (p.x < self.x0 ||
            p.x > self.x1 ||
            p.y < self.y0 ||
            p.y > self.y1) {
            return None;
        }

        Some(HitRecord {
            t: t,
            p: p,
            normal: Vec3::new(0.0,0.0,r.dir.z.signum() * -1.0), // normal along z axis, but opposite of ray dir
            u: (p.x - self.x0) / (self.x1 - self.x0),
            v: (p.y - self.y0) / (self.y1 - self.y0),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hit_unit_sphere() {
        let s = Sphere {
          center: Vec3::new(0.0, 0.0, -1.0),
          radius: 1.0,
        };
        let r = Ray {
          origin: Vec3::new(0.0, 0.0, 1.0),
          dir: Vec3::new(0.0, 0.0, -1.0),
        };
        let expected = HitRecord {
          t: 1.0,
          p: Vec3::new(0.0, 0.0, 0.0),
          normal: Vec3::new(0.0, 0.0, 1.0),
          u: 0.0,
          v: 0.0,
        };
        let res = s.hit(&r, -100.0, 100.0);
        assert!(res.clone().is_some());
        assert!(res.clone().unwrap().t == expected.t);
        assert!(res.clone().unwrap().p == expected.p);
        assert!(res.clone().unwrap().normal == expected.normal);
    }
}
