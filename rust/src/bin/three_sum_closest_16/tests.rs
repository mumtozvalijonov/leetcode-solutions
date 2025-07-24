use super::solution::Solution;

macro_rules! three_sum_closest_tests {
    (
        $(
            $name:ident : $input:expr => $expected:expr
        ),* $(,)?
    ) => {
        $(
            #[test]
            fn $name() {
                let (nums, target) = $input;
                let expected = $expected;
                assert_eq!(Solution::three_sum_closest(nums, target), expected);
            }
        )*
    };
}

three_sum_closest_tests! {
    test_1: (vec![-1, 2, 1, -4], 1) => 2,
    test_2: (vec![0, 0, 0], 1) => 0,
    test_3: (vec![10, 20, 30, 40, 50, 60, 70, 80, 90], 1) => 60
}
