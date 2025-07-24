// 1092. Shortest Common Supersequence

struct Solution;

fn main() {
    println!(
        "{}",
        Solution::shortest_common_supersequence("bbababbb".to_owned(), "bbbaaaba".to_owned())
    );
}

impl Solution {
    fn _build_dp(str1: &String, str2: &String) -> Vec<Vec<u16>> {
        let str1 = str1.as_bytes();
        let str2 = str2.as_bytes();
        let mut dp = vec![vec![0; str2.len() + 1]; str1.len() + 1];
        for i in (0..str1.len()).rev() {
            for j in (0..str2.len()).rev() {
                if str1[i] == str2[j] {
                    dp[i][j] = 1 + dp[i + 1][j + 1];
                } else {
                    dp[i][j] = std::cmp::max(dp[i + 1][j], dp[i][j + 1]);
                }
            }
        }
        dp
    }

    pub fn shortest_common_supersequence(str1: String, str2: String) -> String {
        let dp = Self::_build_dp(&str1, &str2);
        let (mut i, mut j) = (0, 0);
        let str1 = str1.as_bytes();
        let str2 = str2.as_bytes();
        let mut result: Vec<u8> = Vec::with_capacity(str1.len() + str2.len());

        while i < str1.len() && j < str2.len() {
            let (below, right) = (dp[i + 1][j], dp[i][j + 1]);
            if right > below {
                result.push(str2[j]);
                j += 1;
            } else if right < below {
                result.push(str1[i]);
                i += 1;
            } else {
                result.push(str1[i]);
                if i < str1.len() && str1[i] == str2[j] {
                    j += 1;
                }
                i += 1;
            }
        }
        while i < str1.len() {
            result.push(str1[i]);
            i += 1;
        }
        while j < str2.len() {
            result.push(str2[j]);
            j += 1;
        }
        String::from_utf8(result).unwrap()
    }
}
