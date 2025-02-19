// 2375. Construct Smallest Number From DI String

use std::collections::LinkedList;

struct Solution;

fn main() {
    println!("{}", Solution::smallest_number("DDD".to_string()));
}

impl Solution {
    pub fn smallest_number(pattern: String) -> String {
        let mut solution_vector: LinkedList<(char, usize)> = LinkedList::from([('I', 1)]);
        for c in pattern.chars() {
            match c {
                'I' => {
                    solution_vector.push_back(('I', solution_vector.len() + 1));
                }
                'D' => {
                    let mut temp_stack: Vec<(char, usize)> = Vec::new();
                    while let Some((c, val)) = solution_vector.pop_back() {
                        temp_stack.push((c, val));
                        if c == 'I' {
                            break;
                        }
                    }
                    let cur_val = solution_vector.len() + 1;

                    while let Some((c, prev_val)) = temp_stack.pop() {
                        solution_vector.push_back((c, prev_val + 1));
                    }
                    solution_vector.push_back(('D', cur_val));
                }
                _ => {
                    panic!()
                }
            }
        }

        solution_vector
            .iter()
            .map(|(_, v)| std::char::from_digit(*v as u32, 10).unwrap())
            .collect::<String>()
    }
}
