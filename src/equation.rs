#[derive(Debug, PartialEq)]
pub enum Solution {
    TwoRoots(f64, f64),
    OneRoot(f64),
    NoRealRoots,
}

pub struct QuadraticEquation {
    a: f64,
    b: f64,
    c: f64,
}

impl QuadraticEquation {
    #[must_use]
    pub fn new(a: f64, b: f64, c: f64) -> QuadraticEquation {
        QuadraticEquation { a, b, c }
    }

    pub(crate) fn a(&self) -> f64 {
        self.a
    }
    pub(crate) fn b(&self) -> f64 {
        self.b
    }
    pub(crate) fn c(&self) -> f64 {
        self.c
    }
}
