// 3208. Alternating Groups II
struct Solution;

fn main() {
    println!(
        "{}",
        Solution::number_of_alternating_groups(vec![0, 1, 0, 1, 0], 3)
    );
}

impl Solution {
    pub fn number_of_alternating_groups(colors: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;
        let n = colors.len();
        if n == 0 {
            return 0;
        }
        // Create an iterator that represents the circular array:
        // first, iterate over all colors and then the first k-1 colors.
        let extended = colors.iter().chain(colors.iter().take(k - 1));

        let mut result = 0;
        let mut run_length = 1; // A single element is trivially alternating.

        // Get the first element for comparison.
        let iter = extended.skip(1);
        let mut prev = colors[0];

        for &color in iter {
            // If consecutive elements alternate, extend the run.
            if color != prev {
                run_length += 1;
            } else {
                // Once the alternating streak is broken,
                // count the number of subarrays of length k in this run.
                if run_length >= k {
                    result += run_length - k + 1;
                }
                run_length = 1; // Reset run length with the current element.
            }
            prev = color;
        }

        // Final check for the last run.
        if run_length >= k {
            result += run_length - k + 1;
        }

        result as i32
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_1() {
        assert_eq!(
            Solution::number_of_alternating_groups(vec![0, 1, 0, 1, 0], 3),
            3
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            Solution::number_of_alternating_groups(vec![0, 1, 0, 0, 1, 0, 1], 6),
            2
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(
            Solution::number_of_alternating_groups(vec![1, 1, 0, 1], 4),
            0
        );
    }
}
