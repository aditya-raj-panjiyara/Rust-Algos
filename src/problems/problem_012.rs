// Problem 12: Highly Divisible Triangular Number
// Find the first triangle number with over 500 divisors

fn count_divisors(n: u64) -> usize {
    if n == 0 {
        return 0;
    }
    
    let mut count = 0;
    let limit = (n as f64).sqrt() as u64;
    
    for i in 1..=limit {
        if n % i == 0 {
            count += 1;
            if i != n / i {
                count += 1;
            }
        }
    }
    
    count
}

pub fn solve() -> u64 {
    let mut triangle_num = 0;
    let mut i = 1;
    
    loop {
        triangle_num += i;
        if count_divisors(triangle_num) >= 500 {
            return triangle_num;
        }
        i += 1;
    }
}
