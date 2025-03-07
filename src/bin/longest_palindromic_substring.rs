// 5. Longest Palindromic Substring
struct Solution;

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        if s.is_empty() {
            return s;
        }
        let s_bytes = s.as_bytes();
        let mut start = 0;
        let mut max_len = 0;

        for i in 0..s_bytes.len() {
            // Expand for an odd-length palindrome (single center)
            let (l1, len1) = Self::expand(s_bytes, i, i);
            if len1 > max_len {
                start = l1;
                max_len = len1;
            }

            // Expand for an even-length palindrome (center is between i and i+1)
            if i + 1 < s_bytes.len() {
                let (l2, len2) = Self::expand(s_bytes, i, i + 1);
                if len2 > max_len {
                    start = l2;
                    max_len = len2;
                }
            }
        }
        String::from_utf8(s_bytes[start..start + max_len].to_vec()).unwrap()
    }

    // Expands around the center defined by indices `left` and `right`.
    // Returns a tuple (start_index, length) of the longest palindromic substring.
    fn expand(s: &[u8], left: usize, right: usize) -> (usize, usize) {
        // Convert to isize to handle the decrement safely.
        let mut l = left as isize;
        let mut r = right as isize;
        let n = s.len() as isize;
        while l >= 0 && r < n && s[l as usize] == s[r as usize] {
            l -= 1;
            r += 1;
        }
        // After the loop, l and r have gone one step too far.
        ((l + 1) as usize, (r - l - 1) as usize)
    }
}

fn main() {
    println!("{}", Solution::longest_palindrome("a".to_owned()));
}

#[cfg(test)]
mod tests {
    use super::*;

    // Helper function to check if a string is a palindrome.
    fn is_palindrome(s: &str) -> bool {
        s.chars().eq(s.chars().rev())
    }

    #[test]
    fn test_empty_string() {
        let s = "".to_string();
        let res = Solution::longest_palindrome(s);
        assert_eq!(res, "");
    }

    #[test]
    fn test_single_character() {
        let s = "a".to_string();
        let res = Solution::longest_palindrome(s);
        assert_eq!(res, "a");
    }

    #[test]
    fn test_all_same_characters() {
        let s = "aaaaa".to_string();
        let res = Solution::longest_palindrome(s);
        // When all characters are the same, the entire string is a palindrome.
        assert_eq!(res, "aaaaa");
    }

    #[test]
    fn test_no_palindrome_longer_than_one() {
        // Every single character is a palindrome by itself.
        let s = "abc".to_string();
        let res = Solution::longest_palindrome(s);
        assert!(res.len() == 1);
        assert!(is_palindrome(&res));
    }

    #[test]
    fn test_babad() {
        let s = "babad".to_string();
        let res = Solution::longest_palindrome(s);
        // Both "bab" and "aba" are acceptable answers.
        assert!(res == "bab" || res == "aba");
        assert!(is_palindrome(&res));
        assert_eq!(res.len(), 3);
    }

    #[test]
    fn test_cbbd() {
        let s = "cbbd".to_string();
        let res = Solution::longest_palindrome(s);
        // The expected longest palindrome is "bb"
        assert_eq!(res, "bb");
        assert!(is_palindrome(&res));
    }

    #[test]
    fn test_even_length_palindrome() {
        let s = "abb".to_string();
        let res = Solution::longest_palindrome(s);
        // For "abb", "bb" is the longest palindrome.
        assert_eq!(res, "bb");
        assert!(is_palindrome(&res));
    }
}
