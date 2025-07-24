// 80. Remove Duplicates from Sorted Array II
struct Solution;

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let n = nums.len();
        if n <= 2 {
            return n as i32;
        }
        
        // write pointer starts at 2 because the first two elements are always allowed.
        let mut write = 2;
        for read in 2..n {
            // If the current element is not equal to the element at write - 2,
            // it means it hasn't appeared more than twice.
            if nums[read] != nums[write - 2] {
                nums[write] = nums[read];
                write += 1;
            }
        }
        write as i32
    }
}

fn main() {
    let mut nums = vec![1, 1, 1, 2, 2, 3];
    println!("{}", Solution::remove_duplicates(&mut nums));
    println!("{:?}", nums);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_array() {
        let mut nums = vec![];
        let len = Solution::remove_duplicates(&mut nums);
        assert_eq!(len, 0);
        assert_eq!(nums, vec![]);
    }

    #[test]
    fn test_single_element() {
        let mut nums = vec![1];
        let len = Solution::remove_duplicates(&mut nums);
        assert_eq!(len, 1);
        assert_eq!(nums[..len as usize], [1]);
    }

    #[test]
    fn test_two_elements() {
        let mut nums = vec![1, 1];
        let len = Solution::remove_duplicates(&mut nums);
        assert_eq!(len, 2);
        assert_eq!(nums[..len as usize], [1, 1]);
    }

    #[test]
    fn test_normal_case() {
        let mut nums = vec![1, 1, 1, 2, 2, 3];
        let len = Solution::remove_duplicates(&mut nums);
        // The expected array is [1, 1, 2, 2, 3]
        assert_eq!(len, 5);
        assert_eq!(nums[..len as usize], [1, 1, 2, 2, 3]);
    }

    #[test]
    fn test_already_valid() {
        let mut nums = vec![0, 0, 1, 1, 2, 2, 3, 3];
        let len = Solution::remove_duplicates(&mut nums);
        // The array already satisfies the requirement.
        assert_eq!(len, 8);
        assert_eq!(nums[..len as usize], [0, 0, 1, 1, 2, 2, 3, 3]);
    }

    #[test]
    fn test_mixed_case() {
        let mut nums = vec![0, 0, 0, 0, 1, 1, 1, 1, 2, 2, 3, 4, 4, 4];
        let len = Solution::remove_duplicates(&mut nums);
        // Expected result: [0, 0, 1, 1, 2, 2, 3, 4, 4]
        assert_eq!(len, 9);
        assert_eq!(nums[..len as usize], [0, 0, 1, 1, 2, 2, 3, 4, 4]);
    }
}