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

