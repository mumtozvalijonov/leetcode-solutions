use crate::solution::Solution;

mod solution;

#[cfg(test)]
mod tests;

fn main() {
    println!(
        "{:?}",
        Solution::intersection(vec![1, 2, 2, 3], vec![2, 3, 4, 4])
    )
}
