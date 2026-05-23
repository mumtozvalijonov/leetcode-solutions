use rstest::rstest;

use crate::solution::Solution;

#[rstest]
#[case(43261596, 964176192)]
#[case(2147483644, 1073741822)]
fn test_solution(#[case] input: i32, #[case] expected: i32) {
    assert_eq!(Solution::reverse_bits(input), expected)
}
