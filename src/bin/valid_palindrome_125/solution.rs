pub struct Solution;

impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let s = s.as_bytes();
        let mut left = 0;
        let mut right = s.len() - 1;

        while left < right {
            let (c_left, c_right) = (s[left].to_ascii_lowercase(), s[right].to_ascii_lowercase());
            if c_left.is_ascii_alphanumeric() && c_right.is_ascii_alphanumeric() {
                if c_left != c_right {
                    return false;
                } else {
                    left += 1;
                    right -= 1;
                }
            } else {
                if !c_left.is_ascii_alphanumeric() {
                    left += 1;
                }
                if !c_right.is_ascii_alphanumeric() {
                    right -= 1;
                }
            }
        }
        true
    }
}
