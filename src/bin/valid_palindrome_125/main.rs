use crate::solution::Solution;

mod solution;

#[cfg(test)]
mod tests;

fn main() {
    Solution::is_palindrome("s".to_owned());
}
