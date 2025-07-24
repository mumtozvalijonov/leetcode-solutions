// 70. Climbing Stairs

struct Solution;

fn main() {
    println!("{}", Solution::climb_stairs(44));
}

impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        if n <= 2 {
            return n;
        }
        let mut solutions_vec = Vec::with_capacity(n as usize);
        solutions_vec.push(1);
        solutions_vec.push(2);

        for i in 2..n {
            solutions_vec.push(solutions_vec[i as usize - 1] + solutions_vec[i as usize - 2]);
        }

        solutions_vec.pop().unwrap()
    }
}
