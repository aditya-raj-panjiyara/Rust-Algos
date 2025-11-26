use num_bigint::BigInt;
use num_traits::{Zero, ToPrimitive};
// Problem 11: Summation of Primes
// Sum of digit the number 2^1000
pub fn solve() -> u32 {
    let mut num = BigInt::from(2).pow(1000);
    let mut sum: u32 = 0;

    while num > BigInt::zero() {
        let digit = (&num % 10u32).to_u32().unwrap();
        sum += digit;
        num /= 10u32;
    }

    sum
}

