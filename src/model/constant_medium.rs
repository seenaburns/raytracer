extern crate rand;

use vec3::Vec3;
use ray::Ray;
use model::bvh::{AABB, BoundingBox};
use model::hitable::{HitRecord, Hitable};

use std::f64;
use rand::random;

pub struct ConstantMedium<H: Hitable + BoundingBox> {
    bounding: H,
    density: f64,
}

impl<H: Hitable + BoundingBox> ConstantMedium<H> {
    pub fn new(h: H, density: f64) -> Self {
        ConstantMedium {
            bounding: h,
            density: density
        }
    }

    // Find hit when ray enters and exits bounding volume
    // Inverting the ray is not sufficient for concave volumes
    pub fn find_bounding_hits(&self, r: &Ray) -> Option<(HitRecord, HitRecord)> {
        let hit_min = self.bounding.hit(r,f64::NEG_INFINITY,f64::INFINITY);

        let inverted_ray = Ray::new(r.origin, -r.dir);
        let hit_max = self.bounding.hit(&inverted_ray,f64::NEG_INFINITY,f64::INFINITY);
        // Because inverted ray is going in negative direction, t for inverted ray = -t for ray
        let hit_max = hit_max.map(|h| HitRecord { t: -h.t, .. h });

        hit_min.and_then(|hmin| hit_max.map(|hmax| (hmin,hmax)))
    }
}

impl<H: Hitable + BoundingBox> Hitable for ConstantMedium<H> {
    // For a constant volume:
    // 1. find minimum hit with the bounding hitable along the ray
    // 2. find the max hit
    // 3. generate some hit distance stochastically based on density, if hit distance > (max-min),
    //    ray exitted the volume
    //
    // The book seems to make an assumption that H is a thin surface, so to find max it only looks
    // at min_hit + epsilon, which would immediately return if H is solid. Instead I invert the
    // ray, which is cleaner, but has its own shortcomings when H may be concave.
    //
    // Note this also doesn't consider any internal refraction
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        match self.find_bounding_hits(r) {
            Some((hmin, hmax)) => {
                // Edge case where object is outside of (t_min, t_max)
                if hmin.t > t_max || hmax.t < t_min {
                    return None
                }

                // Make sure returned t not outside (t_min,t_max)
                let result_min = hmin.t.max(t_min);
                let result_max = hmax.t.min(t_max);

                let distance_inside_boundary = ((result_max - result_min)*r.dir).length();
                let hit_distance = -(1.0/self.density) * random::<f64>().ln();

                if hit_distance < distance_inside_boundary {
                    let t = result_min + hit_distance / r.dir.length();
                    Some(HitRecord {
                        t: t,
                        p: r.point_at_parameter(t),
                        normal: Vec3::new(1.0,0.0,0.0), // arbitrary
                        u: 0.0, // arbitrary
                        v: 0.0, // arbitrary
                    })
                } else {
                    None
                }
            }
            None => None
        }
    }
}

impl<H: Hitable + BoundingBox> BoundingBox for ConstantMedium<H> {
    fn bounding_box(&self) -> AABB {
        self.bounding.bounding_box()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use model::sphere::Sphere;

    #[test]
    fn test_find_bounding_hits() {
        let s = Sphere::unit_sphere();
        let m = ConstantMedium {
            bounding: s,
            density: 0.0,
        };
        let r = Ray {
          origin: Vec3::new(0.0, 0.0, -5.0),
          dir: Vec3::new(0.0, 0.0, 1.0),
        };

        let res = m.find_bounding_hits(&r);
        println!("{:?}", res);

        assert!(res.clone().is_some());
        assert!(res.clone().unwrap().0.p == Vec3::new(0.0,0.0,-1.0));
        assert!(res.clone().unwrap().1.p == Vec3::new(0.0,0.0,1.0));
        assert!(res.clone().unwrap().0.t == 4.0);
        assert!(res.clone().unwrap().1.t == 6.0);
    }
}
