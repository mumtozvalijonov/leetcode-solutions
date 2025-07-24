use solution::Solution;

mod solution;

#[cfg(test)]
mod tests;

fn main() {
    println!("{}", Solution::add_digits(32));
}
