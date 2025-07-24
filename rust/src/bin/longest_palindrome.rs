use std::collections::HashSet;

struct Solution;

fn main() {
    println!("{}", Solution::longest_palindrome("a".to_string()));
}

impl Solution {
    pub fn longest_palindrome(s: String) -> i32 {
        let (odd_set, result): (HashSet<char>, i32) =
            s.chars()
                .fold((HashSet::new(), 0), |(mut set, mut count), c| {
                    if set.contains(&c) {
                        count += 2;
                        set.remove(&c);
                    } else {
                        set.insert(c);
                    }
                    (set, count)
                });
        result + !odd_set.is_empty() as i32
    }
}
