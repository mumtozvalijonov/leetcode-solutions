use super::solution::Solution;

#[test]
fn test_1() {
    assert_eq!(Solution::single_number(vec![2, 2, 1]), 1);
}

#[test]
fn test_2() {
    assert_eq!(Solution::single_number(vec![4, 1, 2, 1, 2]), 4)
}

#[test]
fn test_3() {
    assert_eq!(Solution::single_number(vec![1]), 1)
}
