use std::collections::HashMap;

// Problem 14: Longest Collatz Sequence
// Find the first ten digits of the sum of one hundred 50-digit numbers
// n -> n/2     if n is even
// n -> 3n + 1  if n is odd
pub fn solve() -> u32 {
    let limit = 1_000_000;
    let mut seen : HashMap<usize, usize> = HashMap::new();
    
    let mut max_len = 0;
    let mut max_start = 0;
    let mut i = 1;
    
    while i < limit {
       let mut len : usize = 0;
       let mut cur = i;
       // println!("{} {:?}",cur,seen);
       while cur != 1 {
           if seen.contains_key(&cur){
               len = len + seen[&cur];
               break
           }
           if cur % 2 == 0 {
               cur /= 2;
           } else {
               cur = 3 * cur + 1;
           }
           len += 1;  
       }
       seen.insert(i, len);
       if len > max_len {
           max_len = len;
           max_start = i;
       }
       i += 1;
   }
    
    max_start as u32
}