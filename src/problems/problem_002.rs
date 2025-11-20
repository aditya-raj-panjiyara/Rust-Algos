// Problem 2: Even Fibonacci Numbers
// Find the sum of even-valued Fibonacci terms below 4 million

pub fn solve() -> i32 {
    let mut a = 1;
    let mut b = 2;
    let mut sum = 0;
    
    while a <= 4_000_000 {
        if a % 2 == 0 {
            sum += a;
        }
        let next = a + b;
        a = b;
        b = next;
    }
    
    sum
}
