use model::Renderable;
use model::hitable::*;
use rand::random;
use ray::Ray;
use shader::material::Material;
use util::Axis;
use vec3::Vec3;

use std::cmp::Ordering;

use std::iter::FromIterator;

pub trait BoundingBox {
    fn bounding_box(&self) -> AABB;

    fn compare_by_axis(&self, other: &BoundingBox, axis: &Axis) -> Ordering {
        if self.bounding_box().min.get_axis(axis) - other.bounding_box().min.get_axis(axis) < 0.0 {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    }
}

//
// Axis aligned bounding box
//
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AABB {
    pub min: Vec3,
    pub max: Vec3,
}

impl AABB {
    // True if ray intersects AABB at some point between tmin and tmax
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
            result_min = if t0 > result_min { t0 } else { result_min };
            result_max = if t1 < result_max { t1 } else { result_max };

            // Ray components intersect at different areas, miss
            if result_max <= result_min {
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

    pub fn unit_aabb() -> AABB {
        AABB {
            min: Vec3::new(-1.0,-1.0,-1.0),
            max: Vec3::new( 1.0, 1.0, 1.0),
        }
    }

    // Return list of each of the edge points
    pub fn vertices(&self) -> Vec<Vec3> {
        let mut v = vec![];
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let x = (i as i32 as f64)*self.max.x + ((1-i) as i32 as f64)*self.min.x;
                    let y = (j as i32 as f64)*self.max.y + ((1-j) as i32 as f64)*self.min.y;
                    let z = (k as i32 as f64)*self.max.z + ((1-k) as i32 as f64)*self.min.z;
                    v.push(Vec3::new(x,y,z));
                }
            }
        }
        v
    }
}

//
// BVH Tree Data Structure
//

// Trait for joint of Renderable + BoundingBox
// as_bounding_box so trait object can be cast as a BoundingBox
pub trait BVHItem: Renderable + BoundingBox {
    fn as_bounding_box(&self) -> &BoundingBox;
}
impl<T> BVHItem for T where T: Renderable + BoundingBox {
    fn as_bounding_box(&self) -> &BoundingBox { self }
}

pub struct Node {
    pub left: Option<Box<BVHItem>>,
    pub right: Option<Box<BVHItem>>,
    pub bounding_box: AABB,
}

impl Node {
    pub fn new(mut hitables: Vec<Box<BVHItem>>) -> Node {
        // Choose random axis and sort the hitables along that axis
        let axis = Vec::from_iter(Axis::iterator())[(random::<u8>() % 3) as usize];
        hitables.sort_by(|a: &Box<BVHItem>, b: &Box<BVHItem>| {
            // Explicitly dereference otherwise compiler thinks Hitable not implemented
            (**a).compare_by_axis((**b).as_bounding_box(), axis)
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
                let lnode = Node::new( hitables );
                let rnode = Node::new( hitables2 );
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

impl Renderable for Node {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<(HitRecord, &Material)> {
        if self.bounding_box.hit(r, t_min, t_max) {
            // If left/right exist, attempt to hit them
            let lhit = self.left.as_ref().and_then(|h| h.hit(r, t_min, t_max));
            let rhit = self.right.as_ref().and_then(|h| h.hit(r, t_min, t_max));

            match (&lhit, &rhit) {
                (&Some((ref lhitrec, _)), &Some((ref rhitrec, _))) => {
                    if lhitrec.t < rhitrec.t { lhit.clone() } else  { rhit.clone() }
                }
                (&Some(_), &None) => lhit.clone(),
                (&None, &Some(_)) => rhit.clone(),
                (&None, &None) => None,
            }
        } else {
            None
        }
    }
}

impl BoundingBox for Node {
    fn bounding_box(&self) -> AABB {
        self.bounding_box
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unit_aabb_hit() {
        let r = Ray {
            origin: Vec3::new(0.0,0.0,-2.0),
            dir: Vec3::new(0.0,0.0,2.0),
        };
        let res = AABB::unit_aabb().hit(&r, 0.0001, 1000.0);
        assert!(res);
    }

    #[test]
    fn unit_aabb_miss() {
        let r = Ray {
            origin: Vec3::new(0.0,0.0,-2.0),
            dir: Vec3::new(0.0,2.0,2.0),
        };
        let res = AABB::unit_aabb().hit(&r, 0.0001, 1000.0);
        assert!(!res);
    }

    #[test]
    fn surrounding_box() {
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

#[cfg(test)]
mod bvh_tests {
    use super::*;
    use model::sphere::Sphere;
    use model::Model;

    #[test]
    fn unit_bvh() {
        let unit_bvh = Node::new(
            vec![Box::new(Model::new(
                Sphere::unit_sphere(),
                Material::lambertian_constant(Vec3::new(0.5,0.5,0.5))
            ))]
        );
        assert!(unit_bvh.bounding_box == AABB::unit_aabb());
    }

    #[test]
    fn bvh_hit() {
        let unit_bvh = Node::new(
            vec![Box::new(Model::new(
                Sphere::unit_sphere(),
                Material::lambertian_constant(Vec3::new(0.5,0.5,0.5))
            ))]
        );
        let r = Ray {
            origin: Vec3::new(0.0,0.0,-2.0),
            dir: Vec3::new(0.0,0.0,2.0),
        };
        let res = unit_bvh.hit(&r, 0.0001, 1000.0);
        assert!(res.is_some());
    }

    #[test]
    fn bvh_miss() {
        let unit_bvh = Node::new(
            vec![Box::new(Model::new(
                Sphere::unit_sphere(),
                Material::lambertian_constant(Vec3::new(0.5,0.5,0.5))
            ))]
        );
        let r = Ray {
            origin: Vec3::new(0.0,0.0,-2.0),
            dir: Vec3::new(0.0,2.0,2.0),
        };
        let res = unit_bvh.hit(&r, 0.0001, 1000.0);
        assert!(res.is_none());
    }
}

