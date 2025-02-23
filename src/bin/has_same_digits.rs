// 100579. Check If Digits Are Equal in String After Operations I

struct Solution;

fn main() {
    println!("{}", Solution::has_same_digits("1515".to_string()));
}

impl Solution {
    pub fn has_same_digits(s: String) -> bool {
        let mut s = s;
        let mut s = unsafe { s.as_bytes_mut() };

        while s.len() > 2 {
            let size = s.len();
            for i in 0..size - 1 {
                s[i] = (s[i] + s[i + 1] - 2 * b'0') % 10 + b'0';
            }
            s = &mut s[..size - 1];
        }

        s[0] == s[1]
    }
}
