use vec3::Vec3;
use ray::Ray;
use hitable::{Hitable, HitRecord, HitableList};
use rand::random;
use util::Axis;

use std::iter::FromIterator;

// Axis aligned bounding box
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AABB {
    pub min: Vec3,
    pub max: Vec3,
}

impl AABB {
    pub fn hit(&self, r: &Ray, tmin: f64, tmax: f64) -> bool {
        // For each component find intersection with bounding box planes
        // For a given plane, e.g. x = x0, the ray p(t) = A + tB
        // intersects at (x0 - A)/B

        let mut result_min = tmin;
        let mut result_max = tmax;
        for a in Axis::iterator() {
            let inv_d = 1.0 / r.dir.get_axis(a);
            let t0 = (self.min.get_axis(a) - r.origin.get_axis(a)) * inv_d;
            let t1 = (self.max.get_axis(a) - r.origin.get_axis(a)) * inv_d;

            // Swap if other direction (t1 will be min instead)
            let (t0, t1) = if inv_d < 0.0 { (t1, t0) } else { (t0, t1) };

            // Check each component to see if have a more limited region than what's been found
            result_min = if t0 > tmin { t0 } else { tmin };
            result_max = if t1 < tmax { t1 } else { tmax };

            // Ray components intersect at different areas, miss
            if result_max < result_min {
                return false;
            }
        }

        true
    }

    // Calculate the surrounding AABB by taking the min of mins and max of maxes
    pub fn surrounding_box(a: &AABB, b: &AABB) -> AABB {
        AABB {
            min: Vec3::map2(a.min, b.min, &|x,y| x.min(y)),
            max: Vec3::map2(a.max, b.max, &|x,y| x.max(y)),
        }
    }
}

#[derive(Clone)]
pub struct Node {
    pub left: Option<Box<Hitable>>,
    pub right: Option<Box<Hitable>>,
    pub bounding_box: AABB,
}

impl Node {
    pub fn new(l: HitableList) -> Node {
        // Choose random axis and sort the hitables along that axis
        let axis = Vec::from_iter(Axis::iterator())[(random::<u8>() % 3) as usize];
        let mut hitables = l.items;
        hitables.sort_by(|a: &Box<Hitable>, b: &Box<Hitable>| {
            // Explicitly dereference otherwise compiler thinks Hitable not implemented
            (**a).compare_by_axis(&(**b), axis)
        });

        let (left, right, bounding_box);
        match hitables.len() {
            1 => {
                bounding_box = hitables[0].bounding_box();
                left = Some(hitables.remove(0));
                right = None;
            }
            2 => {
                bounding_box = AABB::surrounding_box(
                    &hitables[0].bounding_box(),
                    &hitables[1].bounding_box()
                );
                left = Some(hitables.remove(0));
                right = Some(hitables.remove(0)); // after first remove, second item now at index 0
            }
            n => {
                let hitables2 = hitables.split_off(n/2);
                let lnode = Node::new(HitableList { items: hitables });
                let rnode = Node::new(HitableList { items: hitables2 });
                bounding_box = AABB::surrounding_box(&lnode.bounding_box(), &rnode.bounding_box());
                left = Some(Box::new(lnode));
                right = Some(Box::new(rnode));
            }
        }

        Node {
            left: left,
            right: right,
            bounding_box: bounding_box,
        }

    }
}

impl Hitable for Node {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        if self.bounding_box.hit(r, t_min, t_max) {
            // If left/right exist, attempt to hit them
            let lhit = self.left.clone().and_then(|h| h.hit(r, t_min, t_max));
            let rhit = self.right.clone().and_then(|h| h.hit(r, t_min, t_max));

            match (lhit, rhit) {
                (Some(lhitrec), Some(rhitrec)) => {
                    if lhitrec.t < rhitrec.t { lhit } else  { rhit }
                }
                (Some(_), None) => lhit,
                (None, Some(_)) => rhit,
                (None, None) => None,
            }
        } else {
            None
        }
    }

    fn bounding_box(&self) -> AABB {
        self.bounding_box
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_impl() {
        let min1 = Vec3::new(1.0,2.0,3.0);
        let max1 = Vec3::new(3.0,5.0,7.0);
        let min2 = Vec3::new(-1.0,4.0,1.0);
        let max2 = Vec3::new(1.0,8.0,3.0);
        let a = AABB { min: min1, max: max1 };
        let b = AABB { min: min2, max: max2 };
        let res = AABB::surrounding_box(&a,&b);
        let expected = AABB {
            min: Vec3::new(-1.0,2.0,1.0),
            max: Vec3::new(3.0,8.0,7.0),
        };
        assert!(res == expected)
    }
}

