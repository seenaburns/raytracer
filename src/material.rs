use vec3::{Vec3, random_in_unit_sphere, reflect};
use ray::Ray;
use hitable::HitRecord;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Lambertian {
    pub albedo: Vec3
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Metal {
    pub albedo: Vec3
}

// Allow hitable to reference a material
// Dispatch to the MaterialResponse using scatter
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Material {
    Lambertian  { m: Lambertian },
    Metal       { m: Metal },
}


pub fn scatter(m: Material, r: &Ray, hit: &HitRecord) -> Option<(Vec3, Ray)> {
    match m {
        Material::Lambertian { m } => m.scatter(r, hit),
        Material::Metal { m }      => m.scatter(r, hit)
    }
}

pub trait MaterialResponse {
    // Return attentuation vector and outgoing ray if produced
    fn scatter(&self, r: &Ray, hit: &HitRecord) -> Option<(Vec3, Ray)>;
}

impl MaterialResponse for Lambertian {
    fn scatter(&self, r: &Ray, hit: &HitRecord) -> Option<(Vec3, Ray)> {
        let target = hit.p + hit.normal + random_in_unit_sphere();
        let scattered = Ray::new(hit.p, target - hit.p);
        let attentuation = self.albedo;
        Some((attentuation, scattered))
    }
}

impl MaterialResponse for Metal {
    fn scatter(&self, r: &Ray, hit: &HitRecord) -> Option<(Vec3, Ray)> {
        let v = r.dir.normalized();
        let reflected = reflect(v, hit.normal);
        let scattered = Ray::new(hit.p, reflected);
        let attentuation = self.albedo;
        if scattered.dir.dot(hit.normal) > 0.0 {
            Some((attentuation, scattered))
        } else {
            None
        }
    }
}
