use num_bigint::BigUint;

pub fn sum_of_n(n: u32) -> u32 {
    n * (n + 1) / 2
}

pub fn factorial(n: usize) -> BigUint {
    // Create a range, convert each number to BigUint, then multiply them all
    (1..=n).map(BigUint::from).product()
}

pub fn number_to_words(n: i32) -> String {
    // Basic map for 0-19
    let ones_map = [
        "",
        "one",
        "two",
        "three",
        "four",
        "five",
        "six",
        "seven",
        "eight",
        "nine",
        "ten",
        "eleven",
        "twelve",
        "thirteen",
        "fourteen",
        "fifteen",
        "sixteen",
        "seventeen",
        "eighteen",
        "nineteen",
    ];

    // Map for tens (index 0 and 1 are empty/unused)
    let tens_map = [
        "", "", "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety",
    ];

    let mut result = String::new();

    match n {
        // Handle 1 to 19 directly
        1..=19 => {
            result.push_str(ones_map[n as usize]);
        }
        // Handle 20 to 99
        20..=99 => {
            let tens = n / 10;
            let ones = n % 10;
            result.push_str(tens_map[tens as usize]);
            if ones > 0 {
                result.push_str(ones_map[ones as usize]);
            }
        }
        // Handle 100 to 999
        100..=999 => {
            let hundreds = n / 100;
            let remainder = n % 100;
            result.push_str(ones_map[hundreds as usize]);
            result.push_str("hundred");

            if remainder > 0 {
                result.push_str("and");
                // Recursively handle the tens/ones part
                result.push_str(&number_to_words(remainder));
            }
        }
        // Handle 1000 specifically (or generalized thousands)
        1000 => {
            result.push_str("onethousand");
        }
        _ => {}
    }

    result
}
