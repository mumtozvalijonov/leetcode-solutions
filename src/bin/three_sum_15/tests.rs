use super::solution::Solution;

#[test]
fn test_1() {
    let expected = vec![vec![-1, -1, 2], vec![-1, 0, 1]];
    let mut result = Solution::three_sum(vec![-1, 0, 1, 2, -1, -4]);
    result.sort();
    assert_eq!(result, expected);
}

#[test]
fn test_2() {
    let expected: Vec<Vec<i32>> = vec![];
    let mut result = Solution::three_sum(vec![0, 1, 1]);
    result.sort();
    assert_eq!(result, expected);
}

#[test]
fn test_3() {
    let expected = vec![vec![0, 0, 0]];
    let mut result = Solution::three_sum(vec![0, 0, 0]);
    result.sort();
    assert_eq!(result, expected);
}

#[test]
fn test_4() {
    let expected = vec![
        vec![-10, 5, 5],
        vec![-5, 0, 5],
        vec![-4, 2, 2],
        vec![-3, -2, 5],
        vec![-3, 1, 2],
        vec![-2, 0, 2],
    ];
    let mut result = Solution::three_sum(vec![
        2, -3, 0, -2, -5, -5, -4, 1, 2, -2, 2, 0, 2, -4, 5, 5, -10,
    ]);
    result.sort();
    assert_eq!(result, expected);
}
