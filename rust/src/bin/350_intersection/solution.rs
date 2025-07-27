pub struct Solution;

impl Solution {
    // I tried to implement on-stack version of this task
    pub fn intersect(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let ((nums1, nums1_len), (nums2, nums2_len)) = {
            let (shorter, longer) = if nums1.len() > nums2.len() {
                (&nums1, &nums2)
            } else {
                (&nums2, &nums1)
            };
            let (shorter_size, longer_size) = (shorter.len(), longer.len());
            let shorter =
                shorter
                    .iter()
                    .enumerate()
                    .fold([-1; 1000], |mut copy_arr, (idx, &val)| {
                        copy_arr[idx] = val as i16;
                        copy_arr
                    });

            let longer = longer
                .iter()
                .enumerate()
                .fold([-1; 1000], |mut copy_arr, (idx, &val)| {
                    copy_arr[idx] = val as i16;
                    copy_arr
                });
            ((shorter, shorter_size), (longer, longer_size))
        };

        let mut nums2 = nums2;
        let mut counts: [u16; 1001] = [0; 1001];
        for &num in nums1.iter().filter(|&x| *x >= 0) {
            counts[num as usize] += 1;
        }

        let mut k = 0;
        for i in 0..nums2_len {
            let x = nums2[i];
            let ui = x as usize;
            if counts[ui] > 0 {
                counts[ui] -= 1;
                nums2[k] = x;
                k += 1;
                if k == nums1_len {
                    break;
                }
            }
        }

        nums2[..k].iter().map(|&x| x as i32).collect()
    }
}
