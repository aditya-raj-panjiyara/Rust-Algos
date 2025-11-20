// Problem 07: 10001st prime
// By listing the first six prime numbers: 2, 3, 5, 7, 11, and 13, we can see that the 6th prime is 13. find the 10001st prime
fn is_prime(num: i32) -> bool {
    let limit = (num as f64).sqrt() as i32;
    for i in 2..=limit {
        if num % i == 0 {
            return false;
        }
    }
    true
}   
 
pub fn solve() -> i32 {
    let mut num = 1;
    let mut count = 0;
    while count < 10001 {
        num += 1;
        if is_prime(num) {
            count += 1;
        }
    }
    num
}


