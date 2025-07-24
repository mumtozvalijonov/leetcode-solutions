// 2570. Merge Two 2D Arrays by Summing Values

struct Solution;

impl Solution {
    pub fn merge_arrays(nums1: Vec<Vec<i32>>, nums2: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut result = Vec::with_capacity(nums1.len() + nums2.len());
        let mut iter1 = nums1.into_iter();
        let mut iter2 = nums2.into_iter();

        let mut cur1 = iter1.next();
        let mut cur2 = iter2.next();

        while cur1.is_some() && cur2.is_some() {
            let v1 = cur1.as_ref().unwrap();
            let v2 = cur2.as_ref().unwrap();

            match v1[0].cmp(&v2[0]) {
                std::cmp::Ordering::Less => {
                    result.push(cur1.take().unwrap());
                    cur1 = iter1.next();
                }
                std::cmp::Ordering::Greater => {
                    result.push(cur2.take().unwrap());
                    cur2 = iter2.next();
                }
                _ => {
                    let (x, y) = (cur1.take().unwrap(), cur2.take().unwrap());
                    result.push(vec![x[0], x[1] + y[1]]);
                    cur1 = iter1.next();
                    cur2 = iter2.next();
                }
            }
        }

        if let Some(v) = cur1 {
            result.push(v);
            result.extend(iter1);
        }
        if let Some(v) = cur2 {
            result.push(v);
            result.extend(iter2);
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        let nums1 = vec![vec![1, 2], vec![2, 3], vec![4, 5]];
        let nums2 = vec![vec![1, 4], vec![3, 2], vec![4, 1]];
        let expected = [vec![1, 6], vec![2, 3], vec![3, 2], vec![4, 6]];
        assert_eq!(Solution::merge_arrays(nums1, nums2), expected.to_vec());
    }
}

fn main() {
    Solution::merge_arrays(vec![vec![1, 2], vec![2, 3]], vec![vec![1, 3]]);
}
