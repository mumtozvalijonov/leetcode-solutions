use crate::solution::Solution;

mod solution;

#[cfg(test)]
mod tests;

fn main() {
    Solution::maximum_length(vec![1, 7, 8, 7, 5]);
}
