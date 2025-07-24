// 2405. Optimal Partition of String

struct Solution;

fn main() {
    println!("{}", Solution::partition_string("abacaba".to_string()));
}

impl Solution {
    pub fn partition_string(s: String) -> i32 {
        let mut ans = 1;
        let mut flag = 0;
        for c in s.chars() {
            let bit = 1 << (c as u8 - b'a');
            if flag & bit != 0 {
                ans += 1;
                flag = 0;
            }
            flag |= bit;
        }
        ans
    }
}
