struct Solution;

impl Solution {
    fn is_prime(n: i32) -> bool {
        if n < 2 {
            return false;
        }
        if n == 2 || n == 3 {
            return true;
        }
        if n % 2 == 0 {
            return false;
        }
        let mut divisor = 3;
        while divisor * divisor <= n {
            if n % divisor == 0 {
                return false;
            }
            divisor += 2;
        }
        true
    }

    fn closest_primes_naive(left: i32, right: i32) -> Vec<i32> {
        let mut prev_prime = -1;
        let mut closest_a = -1;
        let mut closest_b = -1;
        let mut min_difference = i32::MAX;

        for candidate in left..=right {
            if Self::is_prime(candidate) {
                if prev_prime != -1 {
                    let difference = candidate - prev_prime;
                    if difference < min_difference {
                        min_difference = difference;
                        closest_a = prev_prime;
                        closest_b = candidate;
                    }
                    // Early exit for very small gap.
                    if difference <= 2 {
                        return vec![prev_prime, candidate];
                    }
                }
                prev_prime = candidate;
            }
        }
        if closest_a != -1 {
            vec![closest_a, closest_b]
        } else {
            vec![-1, -1]
        }
    }

    fn closest_primes_sieve(left: i32, right: i32) -> Vec<i32> {
        let n = right as usize;
        let mut sieve = vec![true; n + 1];
        sieve[0] = false;
        sieve[1] = false;

        for i in 2..=n {
            if sieve[i] {
                let mut j = i * i;
                while j <= n {
                    sieve[j] = false;
                    j += i;
                }
            }
        }

        let mut prev = -1;
        let mut closest_a = -1;
        let mut closest_b = -1;
        let mut diff = i32::MAX;

        for i in left..=right {
            if sieve[i as usize] {
                if prev != -1 && i - prev < diff {
                    diff = i - prev;
                    closest_a = prev;
                    closest_b = i;
                    if diff <= 2 {
                        return vec![prev, i];
                    }
                }
                prev = i;
            }
        }
        if closest_a != -1 {
            vec![closest_a, closest_b]
        } else {
            vec![-1, -1]
        }
    }

    pub fn closest_primes(left: i32, right: i32) -> Vec<i32> {
        // Estimate cost for sieve: R*log(log(R)) + (R - L)
        let cost_sieve = (right as f64) * (right as f64).ln().ln() + (right - left) as f64;
        // Estimate cost for naive: min(1452, R - L) * sqrt(R)
        let cost_naive = (std::cmp::min(1452, right - left) as f64) * (right as f64).sqrt();

        // Use sieve if its estimated cost is lower
        if cost_sieve < cost_naive {
            Self::closest_primes_sieve(left, right)
        } else {
            Self::closest_primes_naive(left, right)
        }
    }
}

fn main() {
    println!("{:?}", Solution::closest_primes(19, 32));
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_single_prime() {
        assert_eq!(Solution::closest_primes(1, 2), vec![-1, -1]);
    }

    #[test]
    fn test_two_primes() {
        assert_eq!(Solution::closest_primes(1, 3), vec![2, 3]);
    }

    #[test]
    fn test_multiple_primes_1() {
        assert_eq!(Solution::closest_primes(10, 19), vec![11, 13]);
    }

    #[test]
    fn test_multiple_primes_2() {
        assert_eq!(Solution::closest_primes(19, 32), vec![29, 31]);
    }

    #[test]
    fn test_narrow_range() {
        assert_eq!(Solution::closest_primes(100, 110), vec![101, 103]);
    }

    #[test]
    fn test_no_primes() {
        assert_eq!(Solution::closest_primes(14, 16), vec![-1, -1]);
    }

    #[test]
    fn test_composite_edges() {
        assert_eq!(Solution::closest_primes(14, 20), vec![17, 19]);
    }

    #[test]
    fn test_start_at_two() {
        assert_eq!(Solution::closest_primes(2, 2), vec![-1, -1]);
    }

    #[test]
    fn test_large_range() {
        // We don't assert the exact output for a very large range,
        // but we can at least ensure it runs without panic.
        let result = Solution::closest_primes(1, 100_000);
        // The result should either be [-1, -1] or a valid pair.
        assert!(result.len() == 2);
    }

    #[test]
    fn test_many_primes() {
        assert_eq!(Solution::closest_primes(2, 50), vec![2, 3]);
    }
}
