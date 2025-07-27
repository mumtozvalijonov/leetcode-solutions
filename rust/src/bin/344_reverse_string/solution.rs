pub struct Solution;

impl Solution {
    pub fn reverse_string(s: &mut [char]) {
        let size = s.len();
        (0..size / 2).for_each(|idx| s.swap(idx, size - 1 - idx));
    }
}
