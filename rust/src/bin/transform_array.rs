// Q1. Transform Array by Parity

struct Solution;

fn main() {
    println!("{:?}", Solution::transform_array(vec![1,5,1,4,2]));
}

impl Solution {
    pub fn transform_array(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;
        let mut zero_idx = 0;
        for i in 0..nums.len() {
            if nums[i] % 2 == 0 {
                nums[zero_idx] = 0;
                zero_idx += 1;
            }
        }
        while zero_idx < nums.len() {
            nums[zero_idx] = 1;
            zero_idx += 1;
        }
        nums
    }
}