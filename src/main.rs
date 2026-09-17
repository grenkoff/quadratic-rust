mod equation;
mod solver;

use std::io;
use equation::{QuadraticEquation, Solution};

fn read_int(prompt: &str) -> i32 {
    loop {
        println!("{prompt}");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Unable to read the line.");

        match input.trim().parse::<i32>() {
            Ok(num) => return num,
            Err(_) => println!("Please enter a valid integer."),
        }
    }
}

fn main() {
    let a = read_int("Enter a: ");
    let b = read_int("Enter b: ");
    let c = read_int("Enter c: ");

    let eq = QuadraticEquation::new(a.into(), b.into(), c.into());

    let result = solver::solve(&eq);

    match result {
        Solution::TwoRoots(x1, x2) => println!("Two roots: x1={x1}, x2={x2}."),
        Solution::OneRoot(x) => println!("One root: x={x}."),
        Solution::NoRealRoots => println!("No real roots."),
    }
}
