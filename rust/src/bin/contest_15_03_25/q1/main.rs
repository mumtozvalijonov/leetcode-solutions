mod solution;

#[cfg(test)]
mod tests;

use solution::Solution;

fn main() {
    println!("{}", Solution::total_numbers(vec![1, 2, 3, 4]));
}
