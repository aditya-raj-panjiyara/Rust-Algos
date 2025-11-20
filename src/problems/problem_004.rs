// Problem 4: Largest Palindrome Product
// Find the largest palindrome made from the product of two 3-digit numbers

fn is_palindrome(num: i32) -> bool {
    let num_str = num.to_string();
    num_str == num_str.chars().rev().collect::<String>()
}

pub fn solve() -> i32 {
    let mut max_palindrome = 0;
    let mut limit = 999;
    
    while limit >= 100 {
        for i in (100..=limit).rev() {
            let product = limit * i;
            if product <= max_palindrome {
                break;
            }
            if is_palindrome(product) {
                max_palindrome = product;
            }
        }
        limit -= 1;
    }
    
    max_palindrome
}
