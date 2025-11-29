// Problem 9: Special Pythagorean triplet
// Find the Pythagorean triplet for which a + b + c = 1000 

pub fn solve() -> u64 {
    for a in 1..1000 {
        for  b in a+1..1000{
            let c = 1000 - a - b;
            if a * a + b * b == c * c {
                return (a * b * c) as u64;
            }
        }
    }
    0
}
