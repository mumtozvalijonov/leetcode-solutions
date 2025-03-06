// 3. Longest Substring Without Repeating Characters

use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let bytes = s.as_bytes();
        if bytes.len() <= 1 {
            return bytes.len() as i32;
        }
        let mut last_seen: HashMap<u8, usize> = HashMap::new();
        let mut start = 0;
        let mut max_len = 0;
        
        for (i, &b) in bytes.iter().enumerate() {
            if let Some(&prev_index) = last_seen.get(&b) {
                if prev_index >= start {
                    max_len = max_len.max(i - start);
                    start = prev_index + 1;
                }
            }
            last_seen.insert(b, i);
        }
        
        max_len.max(bytes.len() - start) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_string() {
        assert_eq!(Solution::length_of_longest_substring("".to_string()), 0);
    }

    #[test]
    fn test_single_character() {
        assert_eq!(Solution::length_of_longest_substring("a".to_string()), 1);
    }

    #[test]
    fn test_no_repeating_characters() {
        assert_eq!(Solution::length_of_longest_substring("abcdef".to_string()), 6);
    }

    #[test]
    fn test_with_repeating_characters() {
        // "abcabcbb" has a longest substring "abc" with length 3
        assert_eq!(Solution::length_of_longest_substring("abcabcbb".to_string()), 3);
    }

    #[test]
    fn test_all_same_characters() {
        // "aaaa" has a longest substring of a single unique character.
        assert_eq!(Solution::length_of_longest_substring("aaaa".to_string()), 1);
    }

    #[test]
    fn test_mixed_characters() {
        // "pwwkew" has a longest substring "wke" with length 3
        assert_eq!(Solution::length_of_longest_substring("pwwkew".to_string()), 3);
    }
}

fn main() {
    println!("{}", Solution::length_of_longest_substring("example".to_owned()));
}
