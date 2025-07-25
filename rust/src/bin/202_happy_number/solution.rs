pub struct Solution;

impl Solution {
    fn get_digit_square_sum(mut n: i32) -> i32 {
        let mut result = 0;
        while n > 0 {
            result += (n % 10).pow(2);
            n /= 10;
        }
        result
    }

    pub fn is_happy(n: i32) -> bool {
        let mut slow = Solution::get_digit_square_sum(n);
        if slow == 1 {
            return true;
        }

        let mut fast = slow;

        loop {
            for _ in 0..2 {
                fast = Solution::get_digit_square_sum(fast);
                if fast == 1 {
                    return true;
                }
                if fast == slow {
                    return false;
                }
            }
            slow = Solution::get_digit_square_sum(slow);
        }
    }
}
