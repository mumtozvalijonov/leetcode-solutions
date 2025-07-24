use super::solution::Solution;

#[test]
fn test_1() {
    assert_eq!(Solution::max_free_time(5, vec![1, 3], vec![2, 5]), 2)
}

#[test]
fn test_2() {
    assert_eq!(
        Solution::max_free_time(10, vec![0, 3, 7, 9], vec![1, 4, 8, 10]),
        6
    )
}
