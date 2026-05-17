pub struct Solution;

impl Solution {
    pub fn title_to_number(column_title: String) -> i32 {
        let mut result = 0;

        for &c in column_title.as_bytes() {
            result = result * 26 + (c - b'A' + 1) as i32;
        }

        result
    }
}
