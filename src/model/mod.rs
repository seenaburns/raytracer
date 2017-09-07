pub mod bvh;
pub mod hitable;

use model::bvh::{AABB, BoundingBox};
use model::hitable::{HitRecord, Hitable};
use ray::Ray;
use shader::material::Material;

pub trait Renderable: Sync + Send {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<(HitRecord, &Material)>;
}

pub struct Model<G, M>
    where G: Hitable,
          M: Material
{
    pub geometry: G,
    pub shader: M,
}

unsafe impl<G, M> Sync for Model<G, M>
    where G: Hitable,
          M: Material
{}

unsafe impl<G, M> Send for Model<G, M>
    where G: Hitable,
          M: Material
{}

impl<G, M> Model<G, M>
    where G: Hitable,
          M: Material
{
    pub fn new(geometry: G, shader: M) -> Model<G, M> {
        Model { geometry: geometry, shader: shader }
    }
}

impl<G, M> Renderable for Model<G, M>
    where G: Hitable,
          M: Material
{
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<(HitRecord, &Material)> {
        self.geometry.hit(r,t_min,t_max).map(|h| (h, &self.shader as &Material))
    }
}

impl<G, M> BoundingBox for Model<G, M>
    where G: Hitable + BoundingBox,
          M: Material
{
    fn bounding_box(&self) -> AABB {
        self.geometry.bounding_box()
    }
}

impl Renderable for Vec<Box<Renderable>> {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<(HitRecord, &Material)> {
        let mut hit = None;
        let mut closest = t_max;
        for x in self {
            match x.hit(r,t_min,closest) {
                Some((h, m)) => {
                    closest = h.t;
                    hit = Some((h, m))
                },
                None => ()
            }
        }
        hit
    }
}
