use vec3::{Vec3, random_in_unit_sphere};
use ray::Ray;
use hitable::HitRecord;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Lambertian {
    pub albedo: Vec3
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Metal {
    pub albedo: Vec3,
    fuzz: f64,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Dielectric {
    index: f64,
}

// Allow hitable to reference a material
// Dispatch to the MaterialResponse using scatter
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Material {
    Lambertian  { m: Lambertian },
    Metal       { m: Metal },
    Dielectric  { m: Dielectric },
}

impl Material {
    // Convenience constructors
    pub fn lambertian(albedo: Vec3) -> Material {
        Material::Lambertian { m:
            Lambertian {
                albedo: albedo
            }
        }
    }

    pub fn metal(albedo: Vec3, fuzz: f64) -> Material {
        Material::Metal { m:
            Metal {
                albedo: albedo,
                fuzz: if fuzz < 1.0 { fuzz } else { 1.0 }
            }
        }
    }

    pub fn dielectric(index: f64) -> Material {
        Material::Dielectric { m:
            Dielectric {
                index: index
            }
        }
    }
}


pub fn scatter(m: Material, r: &Ray, hit: &HitRecord) -> Option<(Vec3, Ray)> {
    match m {
        Material::Lambertian { m } => m.scatter(r, hit),
        Material::Metal { m }      => m.scatter(r, hit),
        Material::Dielectric { m } => m.scatter(r, hit),
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
        let scattered = Ray::new(hit.p, reflected + random_in_unit_sphere() * self.fuzz);
        let attentuation = self.albedo;
        if scattered.dir.dot(hit.normal) > 0.0 {
            Some((attentuation, scattered))
        } else {
            None
        }
    }
}

impl MaterialResponse for Dielectric {
    fn scatter(&self, r: &Ray, hit: &HitRecord) -> Option<(Vec3, Ray)> {
        let reflected = reflect(r.dir, hit.normal);
        let attentuation = Vec3::new(1.0,1.0,1.0);

        let (outward_normal, ni_over_nt) =
            if r.dir.dot(hit.normal) > 0.0 {
                (-hit.normal, self.index)
            } else {
                (hit.normal, 1.0 / self.index)
            };

        match refract(r.dir, outward_normal, ni_over_nt) {
            Some(refracted) => {
                let scattered = Ray::new(hit.p, refracted);
                Some((attentuation, scattered))
            },
            None => None
        }
    }
}

// Other
// Reflect v over normal n
pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - 2.0 * v.dot(n) * n
}

pub fn refract(v: Vec3, n: Vec3, ni_over_nt: f64) -> Option<Vec3> {
    let uv = v.normalized();
    let dt = uv.dot(n);
    let discriminant = 1.0 - ni_over_nt * ni_over_nt * (1.0 - dt * dt);
    if discriminant > 0.0 {
        let refracted = ni_over_nt * (uv - n * dt) - n * discriminant.sqrt();
        Some(refracted)
    } else {
        None
    }
}
