pub struct Solution;

impl Solution {
    pub fn convert_to_title(mut column_number: i32) -> String {
        let mut buf = [0u8; 8];
        let mut idx = 8;

        while column_number > 0 {
            column_number -= 1;

            idx -= 1;
            buf[idx] = b'A' + (column_number % 26) as u8;

            column_number /= 26;
        }

        unsafe { String::from_utf8_unchecked(buf[idx..].to_vec()) }
    }
}
