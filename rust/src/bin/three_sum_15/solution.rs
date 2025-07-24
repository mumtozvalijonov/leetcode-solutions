use std::collections::HashSet;

pub struct Solution;

impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result = Vec::new();
        let mut nums = nums;
        nums.sort();
        for i in 0..nums.len() - 2 {
            if i > 0 && nums[i] == nums[i - 1] {
                continue;
            }
            let cur_val = nums[i];
            let rest = &nums[i + 1..];

            for tuple in Self::two_sum(rest, -cur_val) {
                result.push(vec![cur_val, tuple.0, tuple.1]);
            }
        }
        result
    }

    fn two_sum(nums: &[i32], target: i32) -> Vec<(i32, i32)> {
        let mut result = Vec::new();
        let mut sum_set = HashSet::new();
        let mut used = HashSet::new();
        for num in nums {
            let diff = target - num;
            if sum_set.contains(&diff) & !used.contains(num) & !used.contains(&diff) {
                result.push((diff, *num));
                sum_set.remove(&diff);
                used.insert(diff);
            } else {
                sum_set.insert(num);
            }
        }
        result
    }
}
