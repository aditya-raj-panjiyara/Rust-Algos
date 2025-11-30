use num_bigint::BigUint;
// Problem 16: Power Digit Sum
// What is the sum of the digits of the number 2^(1000)?
pub fn solve() -> BigUint {
    let result = BigUint::from(2u32).pow(1000);
    let sum: u32 = result.to_string().chars().map(|c| c.to_digit(10).unwrap()).sum();
    BigUint::from(sum)
}