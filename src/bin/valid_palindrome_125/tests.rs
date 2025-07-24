use super::solution::Solution;

#[test]
fn test_1() {
    assert!(Solution::is_palindrome(
        "A man, a plan, a canal: Panama".to_owned()
    ))
}

#[test]
fn test_2() {
    assert!(!Solution::is_palindrome("race a car".to_owned()))
}

#[test]
fn test_3() {
    assert!(Solution::is_palindrome(" ".to_owned()))
}
