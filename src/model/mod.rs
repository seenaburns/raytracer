pub mod bvh;
pub mod hitable;

use model::bvh::{AABB, BoundingBox};
use model::hitable::{HitRecord, Hitable};
use ray::Ray;
use shader::material::Material;

pub trait Renderable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<(HitRecord, &Material)>;
}

pub struct Model<G>
    where G: Hitable
{
    pub geometry: G,
    pub shader: Material,
}

impl<G> Model<G>
    where G: Hitable
{
    pub fn new(geometry: G, shader: Material) -> Model<G> {
        Model { geometry: geometry, shader: shader }
    }
}

impl<G> Renderable for Model<G>
    where G: Hitable
{
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<(HitRecord, &Material)> {
        self.geometry.hit(r,t_min,t_max).map(|h| (h, &self.shader))
    }
}

impl<G> BoundingBox for Model<G>
    where G: Hitable + BoundingBox
{
    fn bounding_box(&self) -> AABB {
        self.geometry.bounding_box()
    }
}

impl<G> Renderable for Vec<Model<G>>
    where G: Hitable
{
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<(HitRecord, &Material)> {
        let mut hit = None;
        let mut closest = t_max;
        for x in self {
            match x.geometry.hit(r,t_min,closest) {
                Some(h) => {
                    closest = h.t;
                    hit = Some((h, &x.shader))
                },
                None => ()
            }
        }
        hit
    }
}
