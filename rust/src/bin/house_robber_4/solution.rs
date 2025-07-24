pub struct Solution;

impl Solution {
    fn _can_steal_with_max_capability(nums: &Vec<i32>, cap: i32, k: i32) -> bool {
        let mut i = 0;
        let mut count = 0;
        while i < nums.len() {
            if nums[i] <= cap {
                count += 1;
                i += 2;
            } else {
                i += 1;
            }
        }
        count >= k
    }

    pub fn min_capability(nums: Vec<i32>, k: i32) -> i32 {
        let mut high = *nums.iter().max().unwrap();
        let mut low = *nums.iter().min().unwrap();
        let mut ans = high;

        while low <= high {
            let mid = low + (high - low) / 2;
            
            if Self::_can_steal_with_max_capability(&nums, mid, k) {
                ans = mid;
                high = mid - 1;
            } else {
                low = mid + 1;
            }
        }
        ans
    }
}
