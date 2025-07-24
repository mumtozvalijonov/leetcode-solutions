use crate::solution::Solution;

mod solution;

#[cfg(test)]
mod tests;

fn main() {
    println!("{}", Solution::maximum_length(vec![1, 2, 3, 4, 5], 2));
}
