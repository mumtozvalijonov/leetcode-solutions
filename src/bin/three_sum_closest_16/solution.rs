pub struct Solution;

impl Solution {
    pub fn three_sum_closest(nums: Vec<i32>, target: i32) -> i32 {
        let mut closest_sum = i32::MAX;
        let mut nums = nums;
        nums.sort();

        for i in 0..nums.len() {
            if i > 0 && nums[i] == nums[i - 1] {
                continue;
            }
            let (mut low, mut high) = (i + 1, nums.len() - 1);
            while low < high {
                let cur_sum = nums[i] + nums[low] + nums[high];

                if cur_sum == target {
                    return cur_sum;
                }

                if (cur_sum - target).abs() < (closest_sum - target).abs() {
                    closest_sum = cur_sum;
                }

                if cur_sum < target {
                    low += 1;
                } else {
                    high -= 1;
                }
            }
        }
        closest_sum
    }
}
