// 1492. The kth Factor of n

use std::collections::VecDeque;

struct Solution;

fn main() {
    println!("{}", Solution::kth_factor(12, 6));
}

impl Solution {
    pub fn kth_factor(n: i32, k: i32) -> i32 {
        let midpoint = f32::sqrt(n as f32).floor() as i32;
        let mut v: VecDeque<i32> = VecDeque::new();
        if k > midpoint * 2 {
            return -1;
        }

        for i in (1..=midpoint).rev() {
            if n % i == 0 {
                v.push_front(i);
                let div = n / i;
                if div != i {
                    v.push_back(n / i);
                }
            }
        }
        *v.get((k - 1) as usize).unwrap_or(&-1)
    }
}
