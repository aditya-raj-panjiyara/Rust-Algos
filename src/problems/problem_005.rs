// 2520 is the smallest number that can be divided by each of the numbers from 1 to 10 without any remainder.
// What is the smallest positive number that is divisible with no remainder by all of the numbers from 1 to 20?


pub fn solve() -> i32 {
    let mut num = 20;
    loop {
        let mut i = 1;
        while i <= 20 {
            if num % i != 0 {
                break;
            }
            i += 1;
        }
        if i == 21 {
            return num;
        }
        num += 20;
    }
}
