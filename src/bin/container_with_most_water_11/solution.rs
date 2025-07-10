pub struct Solution;

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let (mut i, mut j) = (0, height.len() - 1);
        let mut result = std::cmp::min(height[i], height[j]) * (j - i) as i32;

        while i < j - 1 {
            if height[i] > height[j] {
                j -= 1;
            } else {
                i += 1;
            }
            result = result.max(std::cmp::min(height[i], height[j]) * (j - i) as i32);
        }

        result
    }
}
