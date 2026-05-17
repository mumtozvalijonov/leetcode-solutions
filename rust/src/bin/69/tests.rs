use rstest::rstest;

use crate::solution::Solution;

#[rstest]
#[case(4, 2)]
#[case(8, 2)]
#[case(16, 4)]
#[case(23, 4)]
#[case(280, 16)]
#[case(2147395599, 46339)]
fn my_sqrt(#[case] input: i32, #[case] expected: i32) {
    assert_eq!(Solution::my_sqrt(input), expected)
}
