use rstest::rstest;

use crate::solution::Solution;

#[rstest]
#[case(vec![1,2,3], vec![1,2,4])]
#[case(vec![4,3,2,1], vec![4,3,2,2])]
#[case(vec![9], vec![1,0])]
#[case(vec![9,9], vec![1,0,0])]
#[case(vec![8,9], vec![9,0])]
fn plus_one(#[case] input: Vec<i32>, #[case] expected: Vec<i32>) {
    assert_eq!(Solution::plus_one(input), expected)
}
