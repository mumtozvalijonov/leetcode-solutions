// 2226. Maximum Candies Allocated to K Children
struct Solution;

impl Solution {
    pub fn maximum_candies(candies: Vec<i32>, k: i64) -> i32 {
        let k = k as u64;
        let candies = candies
            .iter()
            .map(|&x| u32::try_from(x).unwrap())
            .collect::<Vec<u32>>();
        let total_candies: u64 = candies.iter().map(|x| u64::from(*x)).sum();
        if total_candies < k {
            return 0;
        }
        let mut min_candies = 1_u64;
        let mut max_candies = (total_candies / k) as u64;
        let mut cur_candies = (min_candies + max_candies).div_ceil(2);

        // Now run a binary search to find a reasonable k.
        while cur_candies != min_candies {
            let possible_piles_count =
                candies.iter().map(|x| u64::from(*x)).fold(0, |mut sum, i| {
                    sum += i / cur_candies;
                    sum
                });
            if possible_piles_count >= k {
                min_candies = cur_candies;
            } else {
                max_candies = cur_candies - 1;
            }
            cur_candies = (min_candies + max_candies).div_ceil(2);
        }
        cur_candies as i32
    }
}

fn main() {
    println!("{}", Solution::maximum_candies(vec![3, 5, 4], 3))
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_1() {
        assert_eq!(Solution::maximum_candies(vec![5, 8, 6], 3), 5)
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::maximum_candies(vec![2, 5], 11), 0)
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::maximum_candies(vec![3, 5, 4], 3), 3)
    }
}
