use super::solution::Solution;

#[test]
fn test_1() {
    assert_eq!(Solution::repair_cars(vec![4,2,3,1], 10), 16);
}

#[test]
fn test_2() {
    assert_eq!(Solution::repair_cars(vec![5,1,8], 6), 16);
}