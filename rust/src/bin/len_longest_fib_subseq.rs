// 873. Length of Longest Fibonacci Subsequence

use std::collections::{HashMap, HashSet};

struct Solution;

fn main() {
    println!(
        "{}",
        Solution::len_longest_fib_subseq(vec![1, 2, 3, 4, 5, 6, 7, 8])
    );
}

impl Solution {
    pub fn len_longest_fib_subseq(arr: Vec<i32>) -> i32 {
        let items: HashSet<i32> = arr.iter().cloned().collect();
        let mut memo: HashMap<(i32, i32), i32> = HashMap::new();
        let mut max_result = 0;
        for j in 2..arr.len() {
            for i in 1..j {
                let item_k = arr[j] - arr[i];

                if item_k < arr[i] && items.contains(&(arr[j] - arr[i])) {
                    let len = *memo.get(&(item_k, arr[i])).unwrap_or(&2) + 1;
                    memo.insert((arr[i], arr[j]), len);
                    max_result = std::cmp::max(max_result, len);
                }
            }
        }
        if max_result > 2 {
            max_result
        } else {
            0
        }
    }
}
