pub struct Solution;

impl Solution {
    pub fn maximum_length(nums: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;
        let mut result = 0;
        let mut dp = vec![vec![0; k]; k];

        for &num in &nums {
            let cur_mod = num as usize % k;
            for i in 0..k {
                dp[cur_mod][i] = dp[i][cur_mod] + 1;
                result = result.max(dp[cur_mod][i]);
            }
        }
        result
    }
}
