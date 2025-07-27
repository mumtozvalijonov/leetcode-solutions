use crate::solution::Solution;

mod solution;

#[cfg(test)]
mod tests;

fn main() {
    println!(
        "{}",
        Solution::is_subsequence("df".to_owned(), "def".to_owned())
    )
}
