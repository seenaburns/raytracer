use vec3::Vec3;
use ray::Ray;
use model::bvh::{AABB, BoundingBox};
use model::hitable::{HitRecord, Hitable};
use util::Axis;

// Rectangle along main_axis
// off_axis_a and off_axis_b will determine which axis bounds are for (a0,a1,b0,b1)
// E.g. xyrect: main = z, off_a = x, off_b = y
#[derive(Debug)]
pub struct Rect {
    main: Axis,
    off_a: Axis,
    off_b: Axis,
    a0: f64,
    a1: f64,
    b0: f64,
    b1: f64,
    k: f64,
}

// Constructors
impl Rect {
    pub fn xy_rect(
        x0: f64,
        x1: f64,
        y0: f64,
        y1: f64,
        k : f64,
    ) -> Rect {
        Rect {
            main: Axis::Z,
            off_a: Axis::X,
            off_b: Axis::Y,
            a0: x0,
            a1: x1,
            b0: y0,
            b1: y1,
            k : k,
        }
    }

    pub fn yz_rect(
        y0: f64,
        y1: f64,
        z0: f64,
        z1: f64,
        k : f64,
    ) -> Rect {
        Rect {
            main: Axis::X,
            off_a: Axis::Y,
            off_b: Axis::Z,
            a0: y0,
            a1: y1,
            b0: z0,
            b1: z1,
            k : k,
        }
    }

    pub fn xz_rect(
        x0: f64,
        x1: f64,
        z0: f64,
        z1: f64,
        k : f64,
    ) -> Rect {
        Rect {
            main: Axis::Y,
            off_a: Axis::X,
            off_b: Axis::Z,
            a0: x0,
            a1: x1,
            b0: z0,
            b1: z1,
            k : k,
        }
    }
}

impl Hitable for Rect {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        // Find where ray intersects main plane
        let t = (self.k - r.origin.get_axis(&self.main)) / r.dir.get_axis(&self.main);

        if t < t_min || t > t_max {
            return None;
        }

        let p = r.point_at_parameter(t);

        if p.get_axis(&self.off_a) < self.a0 ||
           p.get_axis(&self.off_a) > self.a1 ||
           p.get_axis(&self.off_b) < self.b0 ||
           p.get_axis(&self.off_b) > self.b1 {
            return None;
        }

        Some(HitRecord {
            t: t,
            p: p,
            normal: Vec3::new(0.0,0.0,0.0).set_axis(&self.main, 1.0),
            u: (p.get_axis(&self.off_a) - self.a0) / (self.a1 - self.a0),
            v: (p.get_axis(&self.off_b) - self.b0) / (self.b1 - self.b0),
        })
    }
}

impl BoundingBox for Rect {
    fn bounding_box(&self) -> AABB {
        AABB {
            min: (Vec3::new(0.0,0.0,0.0)
                    .set_axis(&self.main, self.k-0.0001)
                    .set_axis(&self.off_a, self.a0)
                    .set_axis(&self.off_b, self.b0)),
            max: (Vec3::new(0.0,0.0,0.0)
                    .set_axis(&self.main, self.k+0.0001)
                    .set_axis(&self.off_a, self.a1)
                    .set_axis(&self.off_b, self.b1))
        }
    }
}
