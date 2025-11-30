use std::time::Instant;

mod problems;
use problems::*;
pub mod utils;


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
    run_problem(5, "Smallest Multiple", problem_005::solve);
    run_problem(6, "Sum Square Difference", problem_006::solve);
    run_problem(7, "10001st prime", problem_007::solve);
    run_problem(8, "Largest Product in a Grid", problem_008::solve);
    run_problem(9, "Special Pythagorean triplet", problem_009::solve);
    run_problem(10, "Summation of Primes", problem_010::solve);
    run_problem(11, "Power digit sum", problem_011::solve);
    run_problem(12, "Highly Divisible Triangular Number", problem_012::solve);
    run_problem(13, "Large Sum", problem_013::solve);
    run_problem(14, "Longest Collatz Sequence", problem_014::solve);
    run_problem(15, "Lattice Paths", problem_015::solve);
    run_problem(16, "Power Digit Sum", problem_016::solve);
    run_problem(17, "Number Letter Counts", problem_017::solve);
    run_problem(17, "Maximum Path Sum I", problem_018::solve);
}
