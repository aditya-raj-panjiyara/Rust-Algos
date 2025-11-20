// Problem 1: Multiples of 3 or 5
// Find the sum of all multiples of 3 or 5 below 1000

fn sum_of_multiples_till(mul: i32, till: i32) -> i32 {
    let n = (till - 1) / mul;
    let sum = n * (n + 1) / 2;
    mul * sum
}

pub fn solve() -> i32 {
    let till = 1000;
    sum_of_multiples_till(3, till) + sum_of_multiples_till(5, till) - sum_of_multiples_till(15, till)
}
