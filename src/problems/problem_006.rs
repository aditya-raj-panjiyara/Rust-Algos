// Problem 06: Sum Square Difference
// Find the difference between the sum of the squares of the first one hundred natural numbers and the square of the sum.

pub fn solve() -> i32 {
    let num = 100;
    let sum = num * (num + 1) / 2;
    let sum_of_squares = num * (num + 1) * (2 * num + 1) / 6;
    sum * sum - sum_of_squares
}
