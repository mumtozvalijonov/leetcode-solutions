// 88. Merge Sorted Array
struct Solution;

fn main() {
    let mut nums1 = vec![4,0,0,0,0,0];
    let mut nums2 = vec![1,2,3,5,6];
    Solution::merge(&mut nums1, 1, &mut nums2, 5);
    println!("{:?}", nums1);
}

impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let (m, n) = (m as usize, n as usize);
        let mut inserts_from_first = 0;
        let (mut i, mut j) = (0, 0);

        while inserts_from_first < m && j < n {
            if nums1[i] > nums2[j] {
                nums1.pop();
                nums1.insert(i, nums2[j]);
                j += 1;
            } else {
                inserts_from_first += 1;
            }
            i += 1;
        }
        
        while j < n {
            nums1[i] = nums2[j];
            i += 1;
            j += 1;
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge_both_empty() {
        let mut nums1 = vec![];
        let mut nums2 = vec![];
        Solution::merge(&mut nums1, 0, &mut nums2, 0);
        assert_eq!(nums1, vec![]);
    }

    #[test]
    fn test_merge_first_empty() {
        let mut nums1 = vec![0, 0, 0];
        let mut nums2 = vec![1, 2, 3];
        Solution::merge(&mut nums1, 0, &mut nums2, 3);
        assert_eq!(nums1, vec![1, 2, 3]);
    }

    #[test]
    fn test_merge_second_empty() {
        let mut nums1 = vec![1, 2, 3];
        let mut nums2 = vec![];
        Solution::merge(&mut nums1, 3, &mut nums2, 0);
        assert_eq!(nums1, vec![1, 2, 3]);
    }

    #[test]
    fn test_merge_same_length() {
        let mut nums1 = vec![1, 3, 5, 0, 0, 0];
        let mut nums2 = vec![2, 4, 6];
        Solution::merge(&mut nums1, 3, &mut nums2, 3);
        assert_eq!(nums1, vec![1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn test_merge_different_length() {
        let mut nums1 = vec![1, 2, 3, 0, 0, 0, 0];
        let mut nums2 = vec![2, 5, 6, 7];
        Solution::merge(&mut nums1, 3, &mut nums2, 4);
        assert_eq!(nums1, vec![1, 2, 2, 3, 5, 6, 7]);
    }

    #[test]
    fn test_merge_with_duplicates() {
        let mut nums1 = vec![1, 2, 3, 0, 0, 0];
        let mut nums2 = vec![2, 2, 2];
        Solution::merge(&mut nums1, 3, &mut nums2, 3);
        assert_eq!(nums1, vec![1, 2, 2, 2, 2, 3]);
    }
}
