// 100579. Check If Digits Are Equal in String After Operations I

struct Solution;

fn main() {
    println!(
        "{}",
        Solution::has_same_digits("1515".repeat(2000).to_string())
    );
}

impl Solution {
    pub fn has_same_digits(s: String) -> bool {
        let mut digits = s.into_bytes();
        while digits.len() > 2 {
            digits = digits
                .windows(2)
                .map(|pair| (pair[0] - b'0' + pair[1] - b'0') % 10 + b'0')
                .collect();
        }
        digits[0] == digits[1]
    }
}
