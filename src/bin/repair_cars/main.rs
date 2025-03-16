mod solution;

#[cfg(test)]
mod tests;

use solution::Solution;

fn main() {
    println!("{}", Solution::repair_cars(vec![4, 2, 3, 1], 10));
}
