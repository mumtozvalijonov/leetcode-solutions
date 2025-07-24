pub struct Solution;

impl Solution {
    pub fn add_digits(num: i32) -> i32 {
        match num {
            0 => 0,
            n if n % 9 == 0 => 9,
            n => n % 9,
        }
    }
}
