use solution::Solution;

mod solution;

#[cfg(test)]
mod tests;

fn main() {
    println!("{}", Solution::divide_array([1, 2, 3, 4].to_vec()));
}
