// 118. Pascal's Triangle

struct Solution;

fn main() {
    println!("{:?}", Solution::generate(2));
}

impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let mut dp = Vec::from([vec![1]]);

        while dp.len() < num_rows as usize {
            let mut v = Vec::with_capacity(dp.len() + 1);
            v.push(1);

            for i in 0..dp[dp.len() - 1].len() - 1 {
                v.push(dp[dp.len() - 1][i] + dp[dp.len() - 1][i + 1]);
            }
            v.push(1);
            dp.push(v);
        }
        dp
    }
}
