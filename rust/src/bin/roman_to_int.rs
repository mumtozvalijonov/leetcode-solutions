// 13. Roman to Integer

use std::collections::HashMap;


struct Solution {}

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let mut sum = 0;
        let mut prev = 0;
        let hm: HashMap<u8, i32> = HashMap::from([
            (b'M', 1000),
            (b'D', 500),
            (b'C', 100),
            (b'L', 50),
            (b'X', 10),
            (b'V', 5),
            (b'I', 1)
        ]);

        for ch in s.bytes().rev() {
            let cur = *hm.get(&ch).unwrap();           
            sum += if cur >= prev {cur} else {-cur};
            prev = cur;
        }

        sum
    }
}

fn main() {
    println!("{}", Solution::roman_to_int("MIX".to_string()));
}
