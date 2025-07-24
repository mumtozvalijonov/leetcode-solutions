use super::solution::Solution;
use std::collections::HashSet;

macro_rules! remove_subfolders_tests {
    (
        $(
            $name:ident : $input:expr => $expected:expr
        ),* $(,)?
    ) => {
        $(
            #[test]
            fn $name() {
                let folder = $input;
                let folder: Vec<String> = folder.into_iter().map(|x| x.to_owned()).collect();
                let expected = $expected;
                let expected = HashSet::<String>::from_iter(expected.into_iter().map(|x| x.to_owned()));
                let result = HashSet::<String>::from_iter(Solution::remove_subfolders(folder));
                assert_eq!(result, expected);
            }
        )*
    };
}

remove_subfolders_tests! {
    test_1: vec!["/a","/a/b","/c/d","/c/d/e","/c/f"] => vec!["/a","/c/d","/c/f"],
    test_2: vec!["/a","/a/b/c","/a/b/d"] => vec!["/a"],
    test_3: vec!["/a/b/c","/a/b/ca","/a/b/d"] => vec!["/a/b/c","/a/b/ca","/a/b/d"]
}
