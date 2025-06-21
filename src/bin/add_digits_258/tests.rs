use super::solution::Solution;

#[test]
fn test_1() {
    assert_eq!(Solution::add_digits(38), 2)
}

#[test]
fn test_2() {
    assert_eq!(Solution::add_digits(0), 0)
}
