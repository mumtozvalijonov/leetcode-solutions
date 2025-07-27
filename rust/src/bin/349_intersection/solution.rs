pub struct Solution;

impl Solution {
    pub fn intersection(nums1: Vec<i32>, mut nums2: Vec<i32>) -> Vec<i32> {
        // 1) Build constant‐space flag table
        let mut flags = [false; 1001];
        for x in nums1 {
            flags[x as usize] = true;
        }

        // 2) “Read” from nums2[i], “write” matches into nums2[k]
        let mut k = 0;
        for i in 0..nums2.len() {
            let x = nums2[i];
            let ui = x as usize;
            if flags[ui] {
                flags[ui] = false; // only record each value once
                nums2[k] = x; // overwrite in place
                k += 1;
            }
        }

        // 3) Drop the tail we didn’t overwrite
        nums2.truncate(k);
        nums2
    }
}
