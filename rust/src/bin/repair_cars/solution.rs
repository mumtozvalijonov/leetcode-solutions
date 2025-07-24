pub struct Solution;

impl Solution {
    pub fn repair_cars(ranks: Vec<i32>, cars: i32) -> i64 {
        let cars = cars as i64;
        let mut low = 0;
        let mut high: i64 = cars.pow(2) * i64::from(*ranks.iter().min().unwrap());
        let mut ans = high;

        while low <= high {
            let mid = low + (high - low) / 2;
            let mut can_repair = 0;

            for i in 0..ranks.len() {
                can_repair += (mid / ranks[i] as i64).isqrt();
                if can_repair >= cars {
                    break;
                }
            }
            if can_repair >= cars {
                ans = mid;
                high = mid - 1;
            } else {
                low = mid + 1;
            }
        }
        ans
    }
}
