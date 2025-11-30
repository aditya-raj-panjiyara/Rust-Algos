use crate::utils::math::number_to_words;
// Problem 17: Number Letter Counts
// If all the numbers from 1 to 1000 (one thousand) inclusive were written out in words, how many letters would be used?
pub fn solve() -> i32 {
    let mut total_letters: i32 = 0;
    // println!("{}", number_to_words(115));
    for i in 1..=1000 {
        let word = number_to_words(i);
        total_letters += word.len() as i32;
    }
    total_letters
}