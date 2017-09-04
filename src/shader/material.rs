use vec3::{Vec3, random_in_unit_sphere};
use ray::Ray;
use model::hitable::HitRecord;
use shader::texture::*;
use rand::random;

#[derive(Debug, Clone)]
pub struct Lambertian {
    pub albedo: TextureEnum,
}

#[derive(Debug, Clone)]
pub struct Metal {
    pub albedo: Vec3,
    fuzz: f64,
}

#[derive(Debug, Clone)]
pub struct Dielectric {
    index: f64,
}

impl Material {
    // Convenience constructors
    pub fn lambertian_constant(albedo: Vec3) -> Lambertian {
        Material::lambertian(TextureEnum::ConstantTexture(constant_texture(albedo)))
    }

    pub fn lambertian(albedo: TextureEnum) -> Lambertian {
        Lambertian {
            albedo: albedo
        }
    }

    pub fn metal(albedo: Vec3, fuzz: f64) -> Metal {
        Metal {
            albedo: albedo,
            fuzz: if fuzz < 1.0 { fuzz } else { 1.0 }
        }
    }

    pub fn dielectric(index: f64) -> Dielectric {
        Dielectric {
            index: index
        }
    }
}

pub trait Material {
    // Return attentuation vector and outgoing ray if produced
    fn scatter(&self, r: &Ray, hit: &HitRecord) -> Option<(Vec3, Ray)>;
}

impl Material for Lambertian {
    fn scatter(&self, r: &Ray, hit: &HitRecord) -> Option<(Vec3, Ray)> {
        let target = hit.p + hit.normal + random_in_unit_sphere();
        let scattered = Ray::new(hit.p, target - hit.p);
        let attentuation = self.albedo.value(0.0,0.0,&hit.p);
        Some((attentuation, scattered))
    }
}

impl Material for Metal {
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

impl Material for Dielectric {
    fn scatter(&self, r: &Ray, hit: &HitRecord) -> Option<(Vec3, Ray)> {
        let reflected = reflect(r.dir, hit.normal);
        let attentuation = Vec3::new(1.0,1.0,1.0);

        let (outward_normal, ni_over_nt, cosine) =
            if r.dir.dot(hit.normal) > 0.0 {
                let cosine = self.index * r.dir.dot(hit.normal) / r.dir.length();
                (-hit.normal, self.index, cosine)
            } else {
                let cosine = -r.dir.dot(hit.normal) / r.dir.length();
                (hit.normal, 1.0 / self.index, cosine)
            };

        let out_ray = match refract(r.dir, outward_normal, ni_over_nt) {
            Some(refracted) => {
                if random::<f64>() < schlick(cosine, self.index) {
                    // Return REFLECT
                    Ray::new(hit.p, reflected)
                } else {
                    // Return REFRACT
                    Ray::new(hit.p, refracted)
                }
            },
            None => {
                // No refracted ray, return REFLECT
                Ray::new(hit.p, reflected)
            }
        };

        Some((attentuation, out_ray))
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

pub fn schlick(cosine: f64, index: f64) -> f64 {
    let r0 = (1.0 - index) / (1.0 + index);
    let r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use ::vec3::Vec3;

    #[test]
    fn test_reflect() {
        let v = Vec3::new(1.0,-1.0,0.0);
        let n = Vec3::new(0.0,1.0,0.0);
        assert!(reflect(v,n) == Vec3::new(1.0,1.0,0.0))
    }
}
