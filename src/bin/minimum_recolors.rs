// 2379. Minimum Recolors to Get K Consecutive Black Blocks
struct Solution;

fn main() {
    println!("{}", Solution::minimum_recolors("BW".to_owned(), 1));
}

impl Solution {
    pub fn minimum_recolors(blocks: String, k: i32) -> i32 {
        let bytes = blocks.as_bytes();
        let white = b'W';
        let k = k as usize;

        // Count white blocks in the initial window
        let mut white_count = bytes[..k].iter().filter(|&&b| b == white).count();
        let mut result = white_count;

        // Slide the window over the remaining blocks
        for i in k..bytes.len() {
            white_count += (bytes[i] == white) as usize;
            white_count -= (bytes[i - k] == white) as usize;
            result = result.min(white_count);
        }
        result as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_case() {
        let blocks = "WBWBBBW".to_string();
        let k = 2;
        let res = Solution::minimum_recolors(blocks, k);
        assert_eq!(res, 0);
    }

    #[test]
    fn test_all_black() {
        // All blocks are already black so no recoloring is needed.
        let blocks = "BBBBBB".to_string();
        let k = 3;
        let res = Solution::minimum_recolors(blocks, k);
        assert_eq!(res, 0);
    }

    #[test]
    fn test_all_white() {
        // All blocks are white, so you need to recolor every block in any window.
        let blocks = "WWWWWW".to_string();
        let k = 3;
        let res = Solution::minimum_recolors(blocks, k);
        assert_eq!(res, 3);
    }

    #[test]
    fn test_mixed_case() {
        // Mixed case example to check sliding window accuracy.
        let blocks = "BWWBWWBWB".to_string();
        let k = 4;
        let res = Solution::minimum_recolors(blocks, k);
        // By manual checking, the minimum recolors required is 2.
        assert_eq!(res, 2);
    }

    #[test]
    fn test_single_character() {
        // With only one character and k = 1,
        // If it's white, you'll need to recolor it.
        let blocks = "W".to_string();
        let k = 1;
        let res = Solution::minimum_recolors(blocks, k);
        assert_eq!(res, 1);
    }

    #[test]
    fn test_k_equals_length() {
        // When k is the entire length, simply count the white blocks.
        let blocks = "BWBWBW".to_string();
        let k = 6;
        let res = Solution::minimum_recolors(blocks, k);
        // There are 3 white blocks in "BWBWBW".
        assert_eq!(res, 3);
    }
}
