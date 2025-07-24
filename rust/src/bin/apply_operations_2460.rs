struct Solution;

fn main() {
    println!(
        "{:?}",
        Solution::apply_operations(vec![1, 0, 0, 4, 4, 2, 0, 3, 3, 0])
    );
}

impl Solution {
    pub fn apply_operations(mut nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut write = 0;
        let mut i = 0;
        while i < n {
            // If there's an adjacent pair to merge and it's nonzero.
            if i < n - 1 && nums[i] == nums[i + 1] && nums[i] != 0 {
                nums[write] = nums[i] * 2;
                i += 2;  // Skip the next element since it's merged.
                write += 1;
            } else if nums[i] != 0 {
                // Just write the current nonzero element.
                nums[write] = nums[i];
                i += 1;
                write += 1;
            } else {
                // Skip zeros.
                i += 1;
            }
        }
        // Fill the rest of the array with zeros.
        while write < n {
            nums[write] = 0;
            write += 1;
        }
        nums
    }
}
