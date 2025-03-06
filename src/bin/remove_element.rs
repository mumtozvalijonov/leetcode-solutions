// 27. Remove Element
struct Solution;

fn main() {
    let mut v = vec![1, 1, 1, 2, 3, 1];
    let result = Solution::remove_element(&mut v, 1);
    println!("{}", result);
}

impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut end = nums.len();
        let mut i = 0;
        while i < end {
            if nums[i] == val {
                end -= 1;
                nums.swap(i, end);
            } else {
                i += 1;
            }
        }
        end as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_vec() {
        let mut nums = vec![];
        let k = Solution::remove_element(&mut nums, 1);
        assert_eq!(k, 0);
    }

    #[test]
    fn test_no_occurrence() {
        let mut nums = vec![1, 2, 3, 4, 5];
        let k = Solution::remove_element(&mut nums, 6);
        // k should be equal to the original length because no element is removed
        assert_eq!(k, 5);
        // Verify that the first k elements are unchanged
        assert_eq!(&nums[..k as usize], &[1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_all_occurrence() {
        let mut nums = vec![1, 1, 1, 1, 1];
        let k = Solution::remove_element(&mut nums, 1);
        // Since all elements are equal to val, k should be 0.
        assert_eq!(k, 0);
    }

    #[test]
    fn test_some_occurrence() {
        let mut nums = vec![3, 2, 2, 3];
        let k = Solution::remove_element(&mut nums, 3);
        // The expected new length is 2 because two elements (3's) should be removed.
        assert_eq!(k, 2);
        // Verify that none of the first k elements is equal to 3.
        for i in 0..k as usize {
            assert_ne!(nums[i], 3);
        }
    }

    #[test]
    fn test_mixed_elements() {
        let mut nums = vec![0, 1, 2, 2, 3, 0, 4, 2];
        let k = Solution::remove_element(&mut nums, 2);
        // The expected new length is 5.
        assert_eq!(k, 5);
        // Verify that none of the first k elements is equal to 2.
        for i in 0..k as usize {
            assert_ne!(nums[i], 2);
        }
    }
}
