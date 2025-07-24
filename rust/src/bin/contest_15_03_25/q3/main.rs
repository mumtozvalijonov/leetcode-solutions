mod solution;
mod tree;

#[cfg(test)]
mod tests;

use solution::Solution;

fn main() {
    let words = ["jump", "run", "run", "jump", "run"];
    println!(
        "{:?}",
        Solution::longest_common_prefix(words.map(|x| x.to_owned()).to_vec(), 2)
    );
}
