use std::collections::HashSet;

pub struct Solution;

impl Solution {
    pub fn is_valid(word: String) -> bool {
        if word.len() < 3 {
            return false;
        }

        let vowels = HashSet::from(['a', 'e', 'i', 'o', 'u']);
        let mut has_vowel = false;
        let mut has_consonant = false;
        for mut c in word.chars() {
            c.make_ascii_lowercase();
            match c {
                '0'..='9' => {
                    continue;
                }
                'a'..='z' => {
                    if vowels.contains(&c) {
                        has_vowel = true;
                    } else {
                        has_consonant = true;
                    }
                }
                _ => {
                    return false;
                }
            }
        }
        has_vowel & has_consonant
    }
}
