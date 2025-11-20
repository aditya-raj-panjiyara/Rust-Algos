pub fn solve() -> i64 {
    let mut number: i64 = 600851475143;
    let mut largest_prime = 0;
    
    while number % 2 == 0 {
        largest_prime = 2;
        number /= 2;
    }
    
    let mut i = 3;
    while i * i <= number {
        while number % i == 0 {
            largest_prime = i;
            number /= i;
        }
        i += 2;
    }
    
    if number > 1 {
        largest_prime = number;
    }
    
    largest_prime
}
