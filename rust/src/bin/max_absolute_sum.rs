// 1749. Maximum Absolute Sum of Any Subarray

struct Solution;

fn main() {
    println!(
        "{}",
        Solution::max_absolute_sum(vec![
            -3, -5, -3, -2, -6, 3, 10, -10, -8, -3, 0, 10, 3, -5, 8, 7, -9, -9, 5, -8
        ])
    );
}

impl Solution {
    pub fn max_absolute_sum(nums: Vec<i32>) -> i32 {
        let (_, min, max) = nums
            .iter()
            .fold((0, 0, 0), |(mut total, mut min, mut max), val| {
                total += *val;
                max = std::cmp::max(max, total);
                min = std::cmp::min(min, total);
                (total, min, max)
            });
        i32::abs(max - min)
    }
}
