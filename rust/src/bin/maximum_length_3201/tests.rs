use super::solution::Solution;

macro_rules! max_length_tests {
    (
        $(
            $name:ident : $input:expr => $expected:expr
        ),* $(,)?
    ) => {
        $(
            #[test]
            fn $name() {
                let nums = $input;
                let expected = $expected;
                assert_eq!(Solution::maximum_length(nums), expected);
            }
        )*
    };
}

max_length_tests! {
    test_1: vec![1, 2, 3, 4] => 4,
    test_2: vec![1, 2, 1, 1, 2, 1, 2] => 6,
    test_3: vec![1, 3] => 2,
    test_4: vec![1, 0, 1] => 3,
    test_5: vec![1, 1, 1, 1, 1] => 5,
    test_6: vec![1, 7, 8, 7, 5] => 4,
}
