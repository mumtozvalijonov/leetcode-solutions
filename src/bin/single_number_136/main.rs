use crate::solution::Solution;

mod solution;

#[cfg(test)]
mod tests;

fn main() {
    println!("{}", Solution::single_number(vec![1, 2, 1]));
}
