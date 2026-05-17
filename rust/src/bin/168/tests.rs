use rstest::rstest;

use crate::solution::Solution;

#[rstest]
#[case(1, "A")]
#[case(28, "AB")]
#[case(701, "ZY")]
#[case(703, "AAA")]
fn test_convert_to_title(#[case] input: i32, #[case] expected: String) {
    assert_eq!(Solution::convert_to_title(input), expected)
}
