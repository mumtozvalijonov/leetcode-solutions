use crate::solution::Solution;

mod solution;

#[cfg(test)]
mod tests;

fn main() {
    let folder = vec!["/a", "/a/b", "/c/d", "/c/d/e", "/c/f"];
    println!(
        "{:?}",
        Solution::remove_subfolders(folder.into_iter().map(|x| x.to_owned()).collect())
    );
}
