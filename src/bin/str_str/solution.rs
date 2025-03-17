use core::iter;

pub struct Solution;

impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        let needle = needle.as_bytes();
        let haystack = haystack.as_bytes();
        let n = needle.len();
        let m = haystack.len();
        let mut lps: Vec<usize> = iter::repeat(0).take(n).collect();
        let (mut i, mut j) = (1, 0);

        // Build lps of a needle
        while i < n {
            if needle[i] == needle[j] {
                j += 1;
                lps[i] = j;
                i += 1;
            } else if j != 0 {
                j = lps[j - 1];
            } else {
                lps[i] = 0;
                i += 1;
            }
        }

        // Run KMP on haystack
        (i, j) = (0, 0);
        while i < m {
            if haystack[i] == needle[j] {
                i += 1;
                j += 1;

                if j == n {
                    return (i - j) as i32;
                }
            } else if j != 0 {
                j = lps[j - 1];
            } else {
                i += 1;
            }
        }
        -1
    }
}
