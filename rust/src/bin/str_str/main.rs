use solution::Solution;

mod solution;

#[cfg(test)]
mod tests;

fn main() {
    Solution::str_str("leetcode".to_owned(), "leeto".to_owned());
}
