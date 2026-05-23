use rstest::rstest;

use crate::solution::Solution;

#[rstest]
#[case(8, true)]
#[case(121, true)]
#[case(80, false)]
#[case(-121, false)]
fn test_solution(#[case] input: i32, #[case] expected: bool) {
    assert_eq!(Solution::is_palindrome(input), expected);
}
