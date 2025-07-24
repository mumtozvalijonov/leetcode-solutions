// 3066. Minimum Operations to Exceed Threshold Value II

struct Solution {}
fn main() {
    println!("{}", Solution::min_operations(vec![69,89,57,31,84,97,50,38,91,86], 91));
}

impl Solution {
    pub fn min_operations(nums: Vec<i32>, k: i32) -> i32 {
        // We work with i64 for safety.
        let k = k as i64;
        
        // Helper: compute floor_log3(x): largest i such that 3^i <= x.
        fn floor_log3(x: i64) -> usize {
            let mut i = 0;
            let mut p = 1;
            while p * 3 <= x {
                p *= 3;
                i += 1;
            }
            i
        }
        
        // Helper: compute ceil_log3(k): smallest i such that 3^i >= k.
        fn ceil_log3(k: i64) -> usize {
            let mut i = 0;
            let mut p = 1;
            while p < k {
                p *= 3;
                i += 1;
            }
            i
        }
        
        // Determine the number of buckets we need.
        // Let B be the bucket index such that 3^B >= k.
        let b = ceil_log3(k);
        // We allocate buckets 0..(B+2) so that merges from bucket B+1 can be recorded.
        let buckets = b + 2;
        
        // Each bucket will store:
        // - count[i]: the number of elements in bucket i.
        // - rep[i]: a representative value for the bucket (we take the maximum among
        //   those inserted, which is a safe choice for merging carry values).
        let mut count = vec![0_i64; buckets];
        let mut rep = vec![0_i64; buckets];
        
        // Initialize buckets with all numbers that are below k.
        // (Numbers >= k need no merging.)
        for &num in nums.iter() {
            let x = num as i64;
            if x < k {
                let i = floor_log3(x.max(1)); // ensure x>=1
                count[i] += 1;
                rep[i] = rep[i].max(x);
            }
        }
        
        let mut ops = 0_i64;
        // carry will hold a leftover element (its exact value) from the previous bucket.
        let mut carry: Option<i64> = None;
        
        // Process buckets from lowest to highest.
        for i in 0..buckets {
            // First, if there is a carry from the previous bucket and there is at least one
            // element in the current bucket, merge the carry with one element from bucket i.
            if let Some(c) = carry {
                if count[i] > 0 {
                    // Perform a merge: remove one element from bucket i.
                    count[i] -= 1;
                    ops += 1;
                    // The merged value is computed exactly:
                    // merging c (the leftover from a lower bucket) and an element from bucket i
                    // (whose representative value is rep[i]). Since c is chosen as the maximum
                    // from its bucket, the merge result is:
                    let merged = 2 * c + rep[i];
                    // "Round" the merged value down to its bucket.
                    let j = floor_log3(merged.max(1));
                    // Insert the merged result into bucket j.
                    if j < buckets {
                        count[j] += 1;
                        rep[j] = rep[j].max(merged);
                    } else {
                        // If it exceeds our buckets, we can ignore it because it’s >= k.
                    }
                    // Clear the carry.
                }
            }
            
            // Now, merge pairs within bucket i.
            let pairs = count[i] / 2;
            if pairs > 0 {
                ops += pairs;
                // When merging two numbers from bucket i, we assume the worst-case (rounded) scenario:
                // if both numbers are (rounded down to) rep[i], then merged = 2*rep[i] + rep[i] = 3*rep[i].
                let merged = 3 * rep[i];
                let j = floor_log3(merged.max(1));
                if j < buckets {
                    count[j] += pairs;
                    rep[j] = rep[j].max(merged);
                }
                // Remove the paired elements from the bucket.
                count[i] %= 2;
            }
            
            // If one element remains in bucket i, it becomes the carry for merging with the next bucket.
            if count[i] % 2 == 1 {
                carry = Some(rep[i]);
            } else {
                carry = None;
            }
        }
        
        // Finally, if there is a leftover carry that is still below k, we perform one last merge.
        if let Some(c) = carry {
            if c < k {
                ops += 1;
            }
        }
        
        ops as i32
    }
}

