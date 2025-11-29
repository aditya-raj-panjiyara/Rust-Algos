use num_bigint::BigUint;

use crate::utils::math::factorial;
// Problem 14: Latic Path
// find number of paths in a grid of size 20 x 20
pub fn solve() -> BigUint {
    let grid = 20;
    let paths = factorial(grid * 2) / (factorial(grid) * factorial(grid));
    paths
}