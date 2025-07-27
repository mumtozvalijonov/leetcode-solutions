use crate::solution::Solution;

mod solution;

#[cfg(test)]
mod tests;

fn main() {
    println!("{}", Solution::reverse_vowels("leetcode".to_owned()));
}
