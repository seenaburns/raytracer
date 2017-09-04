use std::fmt::Debug;
use vec3::Vec3;

//
// Texture Definition
//
pub trait Texture: Debug + TextureClone {
    fn value(&self, u: f64, v: f64, p: &Vec3) -> Vec3;
}

pub trait TextureClone {
    fn box_clone(&self) -> Box<Texture>;
}

impl<T> TextureClone for T where T: 'static + Texture + Clone {
    fn box_clone(&self) -> Box<Texture> {
         Box::new(self.clone())
    }
}

//
// TextureEnum of all the possible textures
// Enables static dispatching
//
#[derive(Debug, Clone)]
pub enum TextureEnum {
    ConstantTexture(ConstantTexture),
    CheckerTexture(CheckerTexture),
}

impl TextureEnum {
    pub fn value(&self, u: f64, v: f64, p: &Vec3) -> Vec3 {
        match *self {
            TextureEnum::ConstantTexture(ref x) => x.value(u,v,p),
            TextureEnum::CheckerTexture(ref x) => x.value(u,v,p),
        }
    }
}

impl Clone for Box<Texture> {
    fn clone(&self) -> Box<Texture> {
        self.box_clone()
    }
}


#[derive(Debug, Clone)]
pub struct ConstantTexture {
    pub color: Vec3,
}

impl Texture for ConstantTexture {
    fn value(&self, u: f64, v: f64, p: &Vec3) -> Vec3 {
        self.color
    }
}

#[derive(Debug, Clone)]
pub struct CheckerTexture {
    pub odd: Box<Texture>,
    pub even: Box<Texture>,
    pub scale: f64,
}

impl Texture for CheckerTexture {
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

pub fn checker_texture(t0: Box<Texture>, t1: Box<Texture>, scale: f64) -> CheckerTexture {
    CheckerTexture {
        odd: t0,
        even: t1,
        scale: scale,
    }
}
