mod solution;

#[cfg(test)]
mod tests;

use solution::Solution;

fn main() {
    println!(
        "{}",
        Solution::min_capability(vec![2, 7, 1, 3, 5, 1, 2, 1, 4], 3)
    );
}
