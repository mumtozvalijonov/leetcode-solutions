pub struct Solution;

impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        if x == 0 {
            return 0;
        } else if x < 4 {
            return 1;
        }

        let mut result = x;
        let mut div_by = 2;
        loop {
            result >>= 1;
            if result <= div_by {
                break;
            }
            div_by <<= 1;
        }

        let (mut low, mut high) = (result, div_by);
        let mut mid = low + ((high - low) >> 1);

        while low <= high {
            let sqr_mid = (mid as i64) * (mid as i64);
            if sqr_mid > x as i64 {
                high = mid - 1;
            } else if sqr_mid < x as i64 {
                low = mid + 1;
            } else {
                return mid;
            }

            mid = low + ((high - low) >> 1);
        }

        high
    }
}
