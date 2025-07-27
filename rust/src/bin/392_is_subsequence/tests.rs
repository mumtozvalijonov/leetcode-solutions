use rstest::rstest;

use crate::solution::Solution;

#[rstest]
#[case("abc", "ahbgdc", true)]
#[case("axc", "ahbgdc", false)]
fn test_is_subsequence(#[case] s: String, #[case] t: String, #[case] expected: bool) {
    assert_eq!(Solution::is_subsequence(s, t), expected)
}
