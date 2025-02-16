use std::collections::BTreeSet;

struct Solution {}

fn main() {
    let seq = Solution::construct_distanced_sequence(3);
    println!("{:?}", seq);
}

impl Solution {
    pub fn construct_distanced_sequence(n: i32) -> Vec<i32> {
        let mut init_vector = vec![0; (2 * (n - 1) + 1) as usize];
        let mut available_members: BTreeSet<i32> = (1..=n).map(|v| -v).collect();
        Solution::_populate_vector(0, &mut available_members, &mut init_vector);
        init_vector
    }

    fn _populate_vector(
        cur_pos: usize,
        available_members: &mut BTreeSet<i32>,
        vector: &mut Vec<i32>,
    ) -> bool {
        if cur_pos >= vector.len() {
            return true;
        } else if vector[cur_pos] != 0 {
            return Solution::_populate_vector(cur_pos + 1, available_members, vector);
        }
        for member in available_members.iter().cloned().collect::<Vec<i32>>() {
            let second_pos = if member != -1 {
                cur_pos + (-member) as usize
            } else {
                cur_pos
            };
            if second_pos >= vector.len() || vector[second_pos] != 0 {
                continue;
            }
            vector[cur_pos] = -member;
            vector[second_pos] = -member;
            available_members.remove(&member);
            if Solution::_populate_vector(cur_pos + 1, available_members, vector) {
                return true;
            }
            vector[cur_pos] = 0;
            vector[second_pos] = 0;
            available_members.insert(member);
        }
        false
    }
}
