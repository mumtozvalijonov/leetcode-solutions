// 1780. Check if Number is a Sum of Powers of Three

struct Solution;

fn main() {
    println!("{}", Solution::check_powers_of_three(90));
}

impl Solution {
    pub fn check_powers_of_three(n: i32) -> bool {
        let mut n = n;

        while n > 2 {
            if n % 3 == 0 {
                n /= 3;
            } else if (n - 1) % 3 == 0 {
                n = (n - 1) / 3;
            } else {
                break;
            }
        }
        n == 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        assert!(Solution::check_powers_of_three(12));
    }

    #[test]
    fn test_example2() {
        assert!(Solution::check_powers_of_three(91));
    }

    #[test]
    fn test_example3() {
        assert!(!Solution::check_powers_of_three(21));
    }
}
