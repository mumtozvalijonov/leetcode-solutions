use super::solution::Solution;

#[test]
fn test_1() {
    assert_eq!(Solution::divide_array([3,2,3,2,2,2].to_vec()), true)
}

#[test]
fn test_2() {
    assert_eq!(Solution::divide_array([1,2,3,4].to_vec()), false)
}