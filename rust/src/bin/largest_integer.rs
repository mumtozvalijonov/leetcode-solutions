// 3471. Find the Largest Almost Missing Integer

struct Solution;

impl Solution {
    pub fn largest_integer(nums: Vec<i32>, k: i32) -> i32 {
        let n = nums.len();
        let k = k as usize;
        if n < k {
            return -1;
        }

        // Global counts for each number across all windows.
        let mut global_counts = [0; 51];
        // Frequency array for numbers in the current window.
        let mut freq = [0; 51];

        // Process the first window.
        for &num in &nums[0..k] {
            freq[num as usize] += 1;
        }
        // For each possible number, if it's present in the first window, update the global count.
        for i in 0..51 {
            if freq[i] > 0 {
                global_counts[i] += 1;
            }
        }

        // Slide the window over the rest of the array.
        for i in k..n {
            let out = nums[i - k] as usize;
            let new = nums[i] as usize;

            // Update frequency: remove the element that slides out.
            freq[out] -= 1;
            // And add the new element.
            freq[new] += 1;

            // Since there are only 51 possible numbers, iterate over all of them to update global counts.
            // Each window contributes 1 for every number that is present.
            for j in 0..51 {
                if freq[j] > 0 {
                    global_counts[j] += 1;
                }
            }
        }

        // Choose the largest number that appears in exactly one window.
        let mut result = -1;
        for (num, &count) in global_counts.iter().enumerate() {
            if count == 1 {
                result = result.max(num as i32);
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        assert_eq!(Solution::largest_integer(vec![3, 9, 2, 1, 7], 3), 7);
    }

    #[test]
    fn test_example2() {
        assert_eq!(Solution::largest_integer(vec![3, 9, 7, 2, 1, 7], 4), 3);
    }

    #[test]
    fn test_example3() {
        assert_eq!(Solution::largest_integer(vec![0, 0], 1), -1);
        assert_eq!(Solution::largest_integer(vec![0, 0], 2), 0);
    }
}

fn main() {
    println!("{}", Solution::largest_integer(vec![0, 0], 1));
}
