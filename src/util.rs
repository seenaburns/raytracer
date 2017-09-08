use std::slice::Iter;

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
