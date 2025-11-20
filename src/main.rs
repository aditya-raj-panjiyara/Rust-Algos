use std::time::Instant;

mod problems;
use problems::*;

fn run_problem<T: std::fmt::Display>(num: u32, name: &str, solver: impl FnOnce() -> T) {
    println!("\n=== Problem {}: {} ===", num, name);
    let start = Instant::now();
    let result = solver();
    let duration = start.elapsed();
    println!("Answer: {}", result);
    println!("Time: {:?}", duration);
}

fn main() {
    run_problem(1, "Multiples of 3 or 5", problem_001::solve);
    run_problem(2, "Even Fibonacci Numbers", problem_002::solve);
    run_problem(3, "Largest prime Factor", problem_003::solve);
    run_problem(4, "Largest Palindrome Product", problem_004::solve);
    run_problem(11, "Largest Product in a Grid", problem_011::solve);
    run_problem(12, "Highly Divisible Triangular Number", problem_012::solve);
    run_problem(13, "Large Sum", problem_013::solve);
}
