use super::solution::Solution;

#[test]
fn test_1() {
    assert_eq!(Solution::total_numbers(vec![1, 2, 3, 4]), 12);
}

#[test]
fn test_2() {
    assert_eq!(Solution::total_numbers(vec![0, 2, 2]), 2);
}

#[test]
fn test_3() {
    assert_eq!(Solution::total_numbers(vec![6, 6, 6]), 1);
}

#[test]
fn test_4() {
    assert_eq!(Solution::total_numbers(vec![1, 3, 5]), 0);
}
