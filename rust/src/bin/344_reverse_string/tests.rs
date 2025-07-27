use rstest::rstest;

use crate::solution::Solution;

#[rstest]
#[case(vec!['h', 'e', 'l', 'l', 'o'], vec!['o', 'l', 'l', 'e', 'h'])]
fn test(#[case] mut input: Vec<char>, #[case] expected: Vec<char>) {
    Solution::reverse_string(&mut input);
    assert_eq!(input, expected)
}
