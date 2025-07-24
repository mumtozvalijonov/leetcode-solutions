use std::collections::HashMap;

struct Solution;

fn main() {
    println!("{}", Solution::num_tile_possibilities("AAABBC".to_string())); // Output: 8
}

impl Solution {
    fn precompute_factorials(n: usize) -> Vec<i32> {
        let mut fact_vec = vec![1; n + 1];
        for i in 1..=n {
            fact_vec[i] = fact_vec[i - 1] * i as i32;
        }
        fact_vec
    }

    fn count_permutations(
        fact_vec: &Vec<i32>,
        freq_counts: &mut Vec<usize>, // Memoization key: Frequency vector (size <= 7)
        current_idx: usize,
        counts_vector: &Vec<i32>,
        memo: &mut HashMap<Vec<usize>, i32>,
    ) -> i32 {
        if current_idx >= counts_vector.len() {
            let mut total_count = 0;
            let mut divisor = 1;
            for (i, &freq) in freq_counts.iter().enumerate().filter(|(_, &freq)| freq > 0) {
                divisor *= fact_vec[i].pow(freq as u32);
                total_count += i * freq;
            }
            return fact_vec[total_count] / divisor;
        }

        // Memoization check using the frequency-based key
        if let Some(&cached_result) = memo.get(freq_counts) {
            return cached_result;
        }

        let max_count = counts_vector[current_idx];
        let mut result = 0;
        for i in 0..=max_count {
            freq_counts[i as usize] += 1; // Track how many letters appear `i` times
            result += Solution::count_permutations(
                fact_vec,
                freq_counts,
                current_idx + 1,
                counts_vector,
                memo,
            );
            freq_counts[i as usize] -= 1; // Undo the change
        }

        // Store the result in memoization table
        memo.insert(freq_counts.clone(), result);
        result
    }

    pub fn num_tile_possibilities(tiles: String) -> i32 {
        let mut tile_counts = [0; 26]; // Count occurrences of each letter
        for c in tiles.chars() {
            tile_counts[(c as u8 - b'A') as usize] += 1;
        }

        let counts_vector: Vec<i32> = tile_counts.iter().cloned().filter(|&x| x > 0).collect();
        let fact_vec = Solution::precompute_factorials(tiles.len());

        let mut memo = HashMap::new();
        let mut freq_counts = vec![0; 8]; // At most 7 different letters → size 8 to cover all cases
        Solution::count_permutations(&fact_vec, &mut freq_counts, 0, &counts_vector, &mut memo) - 1
    }
}
