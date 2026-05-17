use rstest::rstest;

use crate::solution::Solution;

#[rstest]
#[case("A", 1)]
#[case("AB", 28)]
#[case("ZY", 701)]
#[case("AAA", 703)]
fn test_solution(#[case] input: String, #[case] expected: i32) {
    assert_eq!(Solution::title_to_number(input), expected)
}
