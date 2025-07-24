// 1768. Merge Strings Alternately

struct Solution;

fn main() {
    println!(
        "{}",
        Solution::merge_alternately("ab".to_owned(), "pqrs".to_owned())
    );
}

impl Solution {
    pub fn merge_alternately(word1: String, word2: String) -> String {
        let word1 = word1.as_bytes();
        let word2 = word2.as_bytes();
        let mut result: Vec<u8> = Vec::with_capacity(word1.len() + word2.len());
        
        let min_len = word1.len().min(word2.len());
        for i in 0..min_len {
            result.push(word1[i]);
            result.push(word2[i]);
        }
        
        result.extend_from_slice(&word1[min_len..]);
        result.extend_from_slice(&word2[min_len..]);
        
        String::from_utf8(result).unwrap()
    }
}
