struct Solution;

fn main() {
    let input = vec!["ab", "a"];
    println!(
        "{}",
        Solution::longest_common_prefix(input.iter().map(|s| s.to_string()).collect())
    )
}

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        if strs.len() == 1 {
            return strs.get(0).unwrap().to_string();
        }
        let mut result = strs.get(0).unwrap().as_bytes();

        for word in &strs[1..] {
            let word = word.as_bytes();
            let min_len = std::cmp::min(word.len(), result.len());
            result = &result[..min_len];

            for i in 0..min_len {
                if result[i] != word[i] {
                    result = &result[..i];
                    break;
                }
            }
        }
        String::from_utf8(result.to_vec()).unwrap()
    }
}
