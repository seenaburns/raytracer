use ::vec3::Vec3;

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    pub origin: Vec3,
    pub dir: Vec3
}

impl Ray {
    pub fn new(o: Vec3, d: Vec3) -> Ray {
        Ray { origin: o, dir: d }
    }

    pub fn point_at_parameter(self, t: f64) -> Vec3 {
        self.origin + (self.dir * t)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point_at_parameter() {
        let r = Ray {
            origin: Vec3::new(1.0,0.0,0.0),
            dir: Vec3::new(1.0,2.0,3.0),
        };
        assert!(r.point_at_parameter(0.5) == Vec3::new(1.5,1.0,1.5))
    }
}
