use super::solution::Solution;

#[test]
fn test_1() {
    assert_eq!(Solution::min_capability(vec![2, 3, 5, 9], 2), 5);
}

#[test]
fn test_2() {
    assert_eq!(Solution::min_capability(vec![2, 7, 9, 3, 1], 2), 2);
}
