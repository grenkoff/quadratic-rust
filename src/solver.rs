use crate::equation::{QuadraticEquation, Solution};

/// Solves the quadratic equation.
///
/// # Panics
///
/// Panics if the coefficient `a` is equal to 0.
#[must_use]
pub fn solve(eq: &QuadraticEquation) -> Solution {
    assert!(
        eq.a() != 0.0,
        "The coefficient 'a' cannot be equal to 0 in a quadratic equation."
    );

    let d = discriminant(eq);

    if d > 0.0 {
        let x1 = (-eq.b() + d.sqrt()) / (2.0 * eq.a());
        let x2 = (-eq.b() - d.sqrt()) / (2.0 * eq.a());
        Solution::TwoRoots(x1, x2)
    } else if d == 0.0 {
        let x = -eq.b() / (2.0 * eq.a());
        Solution::OneRoot(x)
    } else {
        Solution::NoRealRoots
    }
}

fn discriminant(eq: &QuadraticEquation) -> f64 {
    eq.b() * eq.b() - 4.0 * eq.a() * eq.c()
}
