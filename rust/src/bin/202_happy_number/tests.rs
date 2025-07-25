use rstest::rstest;

use crate::solution::Solution;

#[rstest]
#[case(19, true)]
#[case(2, false)]
fn is_happy_test(#[case] input: i32, #[case] expected: bool) {
    assert_eq!(Solution::is_happy(input), expected)
}
