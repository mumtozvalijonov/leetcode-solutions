use std::collections::VecDeque;

pub struct Solution;

impl Solution {
    pub fn plus_one(mut digits: Vec<i32>) -> Vec<i32> {
        let size = digits.len();
        for i in (0..size).rev() {
            digits[i] = (digits[i] + 1) % 10;
            if digits[i] != 0 {
                return digits;
            }
        }
        let mut digits: VecDeque<i32> = digits.into();
        digits.push_front(1);
        digits.into()
    }
}
