use crate::solution::Solution;

mod solution;

#[cfg(test)]
mod tests;

fn main() {
    let mut input = vec!['h', 'e', 'l', 'l', 'o'];
    Solution::reverse_string(&mut input);
    println!("{:?}", input);
}
