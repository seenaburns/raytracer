use std::slice::Iter;
use std::f64::consts::PI;

#[derive(Debug)]
pub enum Axis {
    X,
    Y,
    Z,
}

impl Axis {
    pub fn iterator() -> Iter<'static, Axis> {
        static AXIS: [Axis;  3] = [Axis::X, Axis::Y, Axis::Z];
        AXIS.into_iter()
    }
}

pub fn approx_float_eq(f1: f64, f2: f64) -> bool {
    (f1-f2).abs() < 0.000001
}

// Returns (cos(degrees), sin(degrees))
pub fn degrees_to_cos_and_sin(degrees: f64) -> (f64, f64) {
    let radians = (PI / 180.0) * degrees;
    let sin_theta = radians.sin();
    let cos_theta = radians.cos();
    (cos_theta, sin_theta)
}
