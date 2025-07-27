use rstest::rstest;

use crate::solution::Solution;

#[rstest]
#[case("IceCreAm", "AceCreIm")]
#[case("leetcode", "leotcede")]
fn test_reverse_vowels(#[case] input: String, #[case] expected: String) {
    assert_eq!(Solution::reverse_vowels(input), expected)
}
