use crate::solution::Solution;

mod solution;

#[cfg(test)]
mod tests;

fn main() {
    Solution::three_sum(vec![
        2, -3, 0, -2, -5, -5, -4, 1, 2, -2, 2, 0, 2, -4, 5, 5, -10,
    ]);
}
