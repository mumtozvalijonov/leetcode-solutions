use crate::solution::Solution;

mod solution;

#[cfg(test)]
mod tests;

fn main() {
    let paths = [
        vec!["a"],
        vec!["c"],
        vec!["d"],
        vec!["a", "b"],
        vec!["c", "b"],
        vec!["d", "a"],
    ]
    .iter()
    .map(|x| Vec::<String>::from_iter(x.iter().map(|y| y.to_string())))
    .collect();

    println!("{:?}", Solution::delete_duplicate_folder(paths))
}
