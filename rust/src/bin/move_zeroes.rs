struct Solution;

fn main() {
    let mut vec = vec![1, 0, 2, 3, 0, 4, 5, 5, 0, 5];
    Solution::move_zeroes(&mut vec);
    println!("{:?}", vec);
}

impl Solution {
    pub fn move_zeroes(nums: &mut [i32]) {
        let mut write_idx = 0;
        for i in 0..nums.len() {
            if nums[i] != 0 {
                (nums[write_idx], nums[i]) = (nums[i], nums[write_idx]);
                write_idx += 1;
            }
        }
    }
}
