struct Solution;

fn main() {
    let mut v = vec![0, 0, 0, 0, 1, 1, 1, 2, 2, 2, 3, 4, 4, 5];
    let k = Solution::remove_duplicates(&mut v);
    println!("{} {:?}", k, v);
}

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut cur_idx = 1;
        for i in 1..nums.len() {
            if nums[i] > nums[i - 1] {
                nums[cur_idx] = nums[i];
                cur_idx += 1;
            }
        }
        cur_idx as i32
    }
}
