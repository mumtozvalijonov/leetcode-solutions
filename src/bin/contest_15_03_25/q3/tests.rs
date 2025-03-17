use super::solution::Solution;

#[test]
fn test_1() {
    let words = ["jump", "run", "run", "jump", "run"];
    assert_eq!(
        Solution::longest_common_prefix(words.map(|x| x.to_owned()).to_vec(), 2),
        vec![3, 4, 4, 3, 4]
    )
}

#[test]
fn test_2() {
    let words = ["dog", "racer", "car"];
    assert_eq!(
        Solution::longest_common_prefix(words.map(|x| x.to_owned()).to_vec(), 2),
        vec![0, 0, 0]
    )
}
