extern crate rand;

use vec3::Vec3;

use std::fmt::Debug;
use rand::distributions::{IndependentSample, Range};

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

//
// Perlin Texture
//
#[derive(Debug, Clone)]
pub struct PerlinNoise {
    pub rand_float: Vec<f64>,
    pub perm_x: Vec<i32>,
    pub perm_y: Vec<i32>,
    pub perm_z: Vec<i32>,
}

impl PerlinNoise {
    pub fn new() -> PerlinNoise {
        PerlinNoise {
            rand_float: PerlinNoise::generate_rand_float(),
            perm_x: PerlinNoise::generate_perm(),
            perm_y: PerlinNoise::generate_perm(),
            perm_z: PerlinNoise::generate_perm(),
        }
    }

    fn noise(&self, p: &Vec3) -> f64 {
        let uvw = p.map(&|x| x - x.floor());
        let i = p.x.floor() as i32;
        let j = p.y.floor() as i32;
        let k = p.z.floor() as i32;
        let to_index = |x: i32, dx: i32| ((x+dx) & 255) as usize;

        let mut neighbors = [[[0.0f64; 2]; 2]; 2];
        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    let index =
                        self.perm_x[to_index(i,di)] ^
                        self.perm_y[to_index(j,dj)] ^
                        self.perm_z[to_index(k,dk)];
                    neighbors[di as usize][dj as usize][dk as usize] = self.rand_float[index as usize];
                }
            }
        }

        PerlinNoise::trilinear_interpolate(neighbors, uvw.x, uvw.y, uvw.z)
    }

    fn generate_rand_float() -> Vec<f64> {
        let mut v = Vec::with_capacity(256);
        let range = Range::new(0.0,1.0);
        let mut rng = rand::thread_rng();
        for _ in 0..256 {
            v.push(range.ind_sample(&mut rng));
        }
        v
    }

    fn permute(p: &mut Vec<i32>) {
        let mut rng = rand::thread_rng();
        for i in (0..p.len()).rev() {
            let range = Range::new(0,i+1);
            let target = range.ind_sample(&mut rng);
            let tmp = p[i];
            p[i] = p[target];
            p[target] = tmp;
        }
    }

    fn generate_perm() -> Vec<i32> {
        let mut p = Vec::with_capacity(256);
        for i in 0..256 {
            p.push(i as i32);
        }
        PerlinNoise::permute(&mut p);
        p
    }

    fn trilinear_interpolate(c: [[[f64; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
        let mut acc = 0.0;
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let fi = (i as i32 as f64);
                    let fj = (j as i32 as f64);
                    let fk = (k as i32 as f64);
                    acc += (fi*u + (1.0-fi) * (1.0-u)) *
                           (fj*v + (1.0-fj) * (1.0-v)) *
                           (fk*w + (1.0-fk) * (1.0-w)) *
                           c[i as usize][j as usize][k as usize];
                }
            }
        }
        acc
    }
}

impl Texture for PerlinNoise {
    fn value(&self, u: f64, v: f64, p: &Vec3) -> Vec3 {
        Vec3::new(1.0,1.0,1.0) * self.noise(p)
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

pub fn perlin_noise_texture() -> PerlinNoise {
    PerlinNoise::new()
}
