use super::solution::Solution;

#[test]
fn test_1() {
    assert_eq!(Solution::str_str("sadbutsad".to_owned(), "sad".to_owned()), 0)
}

#[test]
fn test_2() {
    assert_eq!(Solution::str_str("leetcode".to_owned(), "leeto".to_owned()), -1)
}