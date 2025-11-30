// Problem 17: Number Letter Counts
// If all the numbers from 1 to 1000 (one thousand) inclusive were written out in words, how many letters would be used?

static TRIANGLE : &str = r#"75
95 64
17 47 82
18 35 87 10
20 04 82 47 65
19 01 23 75 03 34
88 02 77 73 07 63 67
99 65 04 28 06 16 70 92
41 41 26 56 83 40 80 70 33
41 48 72 33 47 32 37 16 94 29
53 71 44 65 25 43 91 52 97 51 14
70 11 33 28 77 73 17 78 39 68 17 57
91 71 52 38 17 14 91 43 58 50 27 29 48
63 66 04 68 89 53 67 30 73 16 69 87 40 31
04 62 98 27 23 09 70 98 73 93 38 53 60 04 23"#;
pub fn solve() -> i32 {
    let l = TRIANGLE.lines()
        .map(|line| {
            line.split_whitespace()
                .map(|num| num.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();
    // let mut sum  = 0;
    let mut dp = l.clone();
    for i in (0..dp.len()-1).rev() {
        for j in 0..dp[i].len() {
            dp[i][j] += dp[i+1][j].max(dp[i+1][j+1]);
        }
    }
    dp[0][0]
}