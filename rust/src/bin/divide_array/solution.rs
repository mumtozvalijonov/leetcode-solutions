pub struct Solution;

impl Solution {
    pub fn divide_array(nums: Vec<i32>) -> bool {
        let mut num_seen_array = [false; 500];
        for num in nums.iter() {
            let idx = (num - 1) as usize;
            num_seen_array[idx] = !num_seen_array[idx];
        }
        !num_seen_array.contains(&true)
    }
}
