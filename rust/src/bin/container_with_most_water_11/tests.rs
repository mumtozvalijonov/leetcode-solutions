use super::solution::Solution;

#[test]
fn test_1() {
    assert_eq!(Solution::max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]), 49)
}

#[test]
fn test_2() {
    assert_eq!(Solution::max_area(vec![1, 1]), 1)
}
