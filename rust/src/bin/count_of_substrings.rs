// 3306. Count of Substrings Containing Every Vowel and K Consonants II
struct Solution;

fn main() {
    println!(
        "{}",
        Solution::count_of_substrings("ieaouqqieaouqq".to_owned(), 1)
    );
}

impl Solution {
    
    fn _count(word: &[u8]) -> i64 {
        0
    }
    
    pub fn count_of_substrings(word: String, k: i32) -> i64 {
        let mut vowel_counts = [0; 5];
        let mut consonants_count = 0;
        let mut result = 0;
        let word = word.as_bytes();
        let (mut i, mut j) = (0, 0);
        let n = word.len();
        let min_substring_len = 5 + k as usize;

        while i < n {
            match word[i] {
                b'a' => vowel_counts[0] += 1,
                b'e' => vowel_counts[1] += 1,
                b'i' => vowel_counts[2] += 1,
                b'o' => vowel_counts[3] += 1,
                b'u' => vowel_counts[4] += 1,
                _ => consonants_count += 1,
            }
            if consonants_count == k + 1 {
                i -= 1;
                consonants_count -= 1;
                
                
            }
            i += 1;
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_1() {
        assert_eq!(Solution::count_of_substrings("aeioqq".to_owned(), 1), 0);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::count_of_substrings("aeiou".to_owned(), 0), 1);
    }

    #[test]
    fn test_3() {
        assert_eq!(
            Solution::count_of_substrings("ieaouqqieaouqq".to_owned(), 1),
            3
        );
    }
}
