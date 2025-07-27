use rstest::rstest;

use crate::solution::Solution;

#[rstest]
#[case(vec![1,2,2,1], vec![2, 2], vec![2])]
#[case(vec![4,9,5], vec![9,4,9,8,4], vec![4,9])]
fn test_intersection(
    #[case] nums1: Vec<i32>,
    #[case] nums2: Vec<i32>,
    #[case] mut expected: Vec<i32>,
) {
    let mut result = Solution::intersection(nums1, nums2);
    result.sort();
    expected.sort();

    assert_eq!(result, expected)
}
