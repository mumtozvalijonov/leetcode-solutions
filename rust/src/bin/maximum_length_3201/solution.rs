pub struct Solution;

impl Solution {
    pub fn maximum_length(nums: Vec<i32>) -> i32 {
        let mut evens = 0;
        let mut odds = 0;
        let mut alts = 0;
        let mut prev = 2; // sentinel ≠ 0 or 1

        for &n in &nums {
            let p = (n % 2) as usize;
            // “Alternation” count: bump whenever parity ≠ previous
            if p != prev {
                alts += 1;
                prev = p;
            }
            // Tally pure‐parity counts
            if p == 0 {
                evens += 1;
            } else {
                odds += 1;
            }
        }

        // The answer is whichever is biggest:
        //   • use all-evens, or all-odds, or the alternating‐runs count
        evens.max(odds).max(alts)
    }
}
