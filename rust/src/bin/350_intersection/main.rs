use crate::solution::Solution;

mod solution;

#[cfg(test)]
mod tests;

fn main() {
    println!("{:?}", Solution::intersect(vec![1, 2, 3, 4], vec![3, 3, 4]))
}
