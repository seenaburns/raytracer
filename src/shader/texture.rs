use std::fmt::Debug;
use vec3::Vec3;

//
// Texture Definition
//
pub trait Texture: Debug + Clone {
    fn value(&self, u: f64, v: f64, p: &Vec3) -> Vec3;
}

//
// Constant Texture
//
#[derive(Debug, Clone)]
pub struct ConstantTexture {
    pub color: Vec3,
}

impl Texture for ConstantTexture {
    fn value(&self, u: f64, v: f64, p: &Vec3) -> Vec3 {
        self.color
    }
}

//
// Checker Texture
// Alternates between two textures in checkerboard pattern
//
#[derive(Debug, Clone)]
pub struct CheckerTexture<T1: Texture, T2: Texture> {
    pub odd: T1,
    pub even: T2,
    pub scale: f64,
}

impl<T1, T2> Texture for CheckerTexture<T1, T2>
    where T1: Texture,
          T2: Texture
{
    fn value(&self, u: f64, v: f64, p: &Vec3) -> Vec3 {
        let sines = p.map(&|x| (x * self.scale).sin());
        let sines = sines.x * sines.y * sines.z;

        if sines < 0.0 {
            self.odd.value(u,v,p)
        } else {
            self.even.value(u,v,p)
        }
    }
}

// Constructors
pub fn constant_texture(c: Vec3) -> ConstantTexture {
    ConstantTexture { color: c }
}

pub fn checker_texture<T1, T2>(t1: T1, t2: T2, scale: f64) -> CheckerTexture<T1,T2>
    where T1: Texture,
          T2: Texture
{
    CheckerTexture {
        odd: t1,
        even: t2,
        scale: scale,
    }
}
