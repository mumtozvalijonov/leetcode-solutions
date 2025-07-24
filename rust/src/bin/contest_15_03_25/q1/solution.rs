use std::collections::HashSet;

pub struct Solution;

impl Solution {
    pub fn total_numbers(digits: Vec<i32>) -> i32 {
        let mut result = 0;
        let mut seen: HashSet<i32> = HashSet::new();
        for i in 0..digits.len() {
            if digits[i] % 2 != 0 {
                continue;
            }

            for j in 0..digits.len() {
                if j == i || digits[j] == 0 {
                    continue;
                }
                for k in 0..digits.len() {
                    if k == j || k == i {
                        continue;
                    }
                    let num = digits[j] * 100 + digits[k] * 10 + digits[i];
                    if !seen.contains(&num) {
                        result += 1;
                    }
                    seen.insert(num);
                }
            }
        }
        result
    }
}
