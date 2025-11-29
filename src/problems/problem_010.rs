// Problem 9: Summation of Primes
// Find summitions of primes numbers below 2 million
pub fn solve() -> u64 {
    let limit = 2_000_000;
    let mut is_prime = vec![true; limit];
    is_prime[0] = false;
    is_prime[1] = false;

    for i in 2..((limit as f64).sqrt() as usize + 1) {
        if is_prime[i] {
            let mut j = i * i;
            while j < limit {
                is_prime[j] = false;
                j += i;
            }
        }
    }

    is_prime.iter()
        .enumerate()
        .filter(|&(_, &p)| p)
        .map(|(i, _)| i as u64)
        .sum()
}

