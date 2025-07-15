use super::solution::Solution;

#[test]
fn test_1() {
    assert!(Solution::is_valid("234Adas".to_owned()))
}

#[test]
fn test_2() {
    assert!(!Solution::is_valid("b3".to_owned()))
}

#[test]
fn test_3() {
    assert!(!Solution::is_valid("a3$e".to_owned()))
}
