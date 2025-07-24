// Q3. Find Minimum Cost to Remove Array Elements

use std::collections::BinaryHeap;

struct Solution;

fn main() {
    /*
    [6,2,8,4] 12
    [2,1,3,3] 5
    [9,1,5] 10
    [12,15,1,15,18] 34
    */
    println!("{}", Solution::min_cost(vec![9, 1, 5]))
}

impl Solution {
    pub fn min_cost(nums: Vec<i32>) -> i32 {
        let mut cost = 0;
        let mut nums = nums;
        nums.reverse();

        while nums.len() > 3 {
            let mut heap = BinaryHeap::from([
                nums.pop().unwrap(),
                nums.pop().unwrap(),
                nums.pop().unwrap(),
            ]);
            let a = heap.pop().unwrap();
            let b = heap.pop().unwrap();

            if let Some(next1) = nums.pop() {
                if next1 <= b {
                    if let Some(next2) = nums.pop() {
                        if next2 > b {
                            nums.push(next2);
                            nums.push(next1);
                            nums.push(a);
                            cost += b;
                            continue;
                        }
                        nums.push(next2);
                    }
                    cost += a;
                    nums.push(next1);
                    nums.push(heap.pop().unwrap());
                } else {
                    nums.push(next1);
                    nums.push(a);
                    cost += b;
                }
            }
        }
        cost += nums.iter().max().unwrap();
        if nums.len() == 3 {
            cost += nums.iter().min().unwrap();
        }
        cost
    }
}
