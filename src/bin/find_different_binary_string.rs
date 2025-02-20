struct Solution;

fn main() {
    let input = vec!["111", "011", "001"];
    println!(
        "{}",
        Solution::find_different_binary_string(input.iter().map(|x| x.to_string()).collect())
    );
}

impl Solution {
    pub fn find_different_binary_string(nums: Vec<String>) -> String {
        let mut bin_array = String::with_capacity(nums.len());
        for (i, num) in nums.iter().enumerate() {
            if num.get(i..i + 1).unwrap() == "0" {
                bin_array.push('1');
            } else {
                bin_array.push('0');
            }
        }
        bin_array
    }
}
