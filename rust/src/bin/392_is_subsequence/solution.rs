pub struct Solution;

impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        let (s, t) = (s.as_bytes(), t.as_bytes());
        let (mut s_ptr, mut t_ptr) = (0, 0);

        while s_ptr < s.len() && t_ptr < t.len() {
            if s[s_ptr] == t[t_ptr] {
                s_ptr += 1;
            }
            t_ptr += 1;
        }
        s_ptr == s.len()
    }
}
