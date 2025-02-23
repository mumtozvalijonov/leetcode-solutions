// 100576. Maximum Sum With at Most K Elements

use std::collections::BinaryHeap;

struct Solution;

fn main() {
    println!(
        "{}",
        Solution::max_sum(
            vec![vec![4, 40, 38, 38, 22, 19, 6, 9, 33, 46, 9]],
            vec![1],
            1
        )
    )
}

impl Solution {
    pub fn max_sum(grid: Vec<Vec<i32>>, limits: Vec<i32>, k: i32) -> i64 {
        if k == 0 {
            return 0;
        }
        let mut result_heap: BinaryHeap<i32> = BinaryHeap::with_capacity(k as usize);
        let mut result: i64 = 0;
        for (row_idx, values) in grid.iter().enumerate() {
            let heap_size = std::cmp::min(limits[row_idx] as usize, values.len());
            if heap_size == 0 {
                continue;
            }
            let mut heap: BinaryHeap<i32> = BinaryHeap::with_capacity(heap_size);
            for item in values {
                if heap.len() == heap.capacity() {
                    if *item < -heap.peek().unwrap() {
                        continue;
                    }
                    heap.pop();
                }
                heap.push(-*item);
            }

            while let Some(item) = heap.pop() {
                let item = -item;
                if result_heap.len() == result_heap.capacity() {
                    if item < -result_heap.peek().unwrap() {
                        continue;
                    }
                    result_heap.pop();
                }
                result_heap.push(-item);
            }
        }

        while let Some(item) = result_heap.pop() {
            result += -item as i64;
        }
        result
    }
}
