// 3468. Find the Number of Copy Arrays

struct Solution;

fn main() {
    println!(
        "{}",
        Solution::count_arrays(
            vec![57, 55, 75],
            vec![vec![60, 106], vec![43, 60], vec![18, 67]]
        )
    );
}

impl Solution {
    pub fn count_arrays(original: Vec<i32>, mut bounds: Vec<Vec<i32>>) -> i32 {
        let n = original.len();
        // The number of valid starting values for copy[0] is initially the length of the interval.
        let mut possible = bounds[0][1] - bounds[0][0] + 1;

        // For each subsequent index, update the allowed interval based on the difference.
        for i in 1..n {
            let diff = original[i] - original[i - 1];
            // Update current bounds to be the intersection of its original interval
            // and the interval derived by shifting the previous bounds by diff.
            bounds[i][0] = std::cmp::max(bounds[i - 1][0] + diff, bounds[i][0]);
            bounds[i][1] = std::cmp::min(bounds[i - 1][1] + diff, bounds[i][1]);

            // Compute the number of possibilities for this position.
            let current_possible = bounds[i][1] - bounds[i][0] + 1;
            possible = std::cmp::min(possible, current_possible);

            // If at any point there are no possibilities, we can exit early.
            if possible <= 0 {
                return 0;
            }
        }
        possible
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        assert_eq!(
            Solution::count_arrays(
                vec![1, 2, 3, 4],
                vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5]],
            ),
            2
        );
    }
}
