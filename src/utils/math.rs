use num_bigint::BigUint;


pub fn sum_of_n(n: u32) -> u32 {
    n * (n + 1) / 2
}

pub fn factorial(n: usize) -> BigUint {
    // Create a range, convert each number to BigUint, then multiply them all
    (1..=n).map(BigUint::from).product()
}
