// 2161. Partition Array According to Given Pivot
use std::collections::VecDeque;

struct Solution;

impl Solution {
    pub fn pivot_array(nums: Vec<i32>, pivot: i32) -> Vec<i32> {
        let mut queue = VecDeque::with_capacity(nums.len());
        let mut cnt_pivot = 0;
        for &num in nums.iter() {
            if num > pivot {
                queue.push_back(num);
            } else if num == pivot {
                cnt_pivot += 1;
            }
        }

        for _ in 0..cnt_pivot {
            queue.push_front(pivot);
        }

        for &num in nums.iter().rev() {
            if num < pivot {
                queue.push_front(num);
            }
        }
        Vec::from(queue)
    }
}

fn main() {
    println!(
        "{:?}",
        Solution::pivot_array(vec![18, 9, 16, 12, 4, 5, 15, 2, 13, 10, 14, 3, 10], 10)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pivot_array() {
        let test_cases = vec![
            (vec![1, 2, 3, 4, 5], 3, vec![1, 2, 3, 4, 5]),
            (vec![5, 4, 3, 2, 1], 3, vec![2, 1, 3, 5, 4]),
            (vec![10, 10, 10], 10, vec![10, 10, 10]),
            (vec![1, 2, 3, 4, 5], 6, vec![1, 2, 3, 4, 5]),
            (vec![5, 4, 3, 2, 1], 0, vec![5, 4, 3, 2, 1]),
        ];

        for (nums, pivot, expected) in test_cases {
            let result = Solution::pivot_array(nums.clone(), pivot);
            assert_eq!(result, expected, "Test case nums: {:?}, pivot: {} failed", nums, pivot);
        }
    }
}
