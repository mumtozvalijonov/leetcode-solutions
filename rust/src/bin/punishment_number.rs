struct Solution {}

fn main() {
    println!("{}", Solution::punishment_number(37));
}

fn _is_punishment_term(m: i32, sum_substr: i32, leftover: i32) -> bool {
    if sum_substr == m && leftover == 0 {
        return true;
    } else if (sum_substr > m) || (leftover == 0) {
        return false;
    }

    for pow in 1..(leftover as f32).log10() as u32 + 2 {
        let divisor = 10_i32.pow(pow);
        if _is_punishment_term(m, sum_substr + leftover % divisor, leftover / divisor) {
            return true;
        }
    }
    false
}

impl Solution {
    pub fn punishment_number(n: i32) -> i32 {
        let mut result = 0;

        for num in 1..n + 1 {
            if _is_punishment_term(num, 0, num * num) {
                result += num * num;
            }
        }
        result
    }
}
