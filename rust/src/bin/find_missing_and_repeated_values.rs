// 2965. Find Missing and Repeated Values

struct Solution;

fn main() {
    println!(
        "{:?}",
        Solution::find_missing_and_repeated_values(vec![
            vec![1, 2],
            vec![2, 3],
        ])
    );
}

impl Solution {
    pub fn find_missing_and_repeated_values(grid: Vec<Vec<i32>>) -> Vec<i32> {
        let mut pool: Vec<bool> = vec![false; grid.len().pow(2)];
        let mut result = vec![0, 0];
        for row in grid {
            for val in row {
                let idx = (val - 1) as usize;
                if pool[idx] {
                    result[0] = val;
                }
                pool[idx] = true;
            }
        }
        result[1] = pool.iter().position(|x| !x).unwrap() as i32 + 1;
        result
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            Solution::find_missing_and_repeated_values(vec![
                vec![1, 2],
                vec![2, 3],
            ]),
            vec![2, 4]
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            Solution::find_missing_and_repeated_values(vec![
                vec![1, 1],
                vec![2, 3],
            ]),
            vec![1, 4]
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(
            Solution::find_missing_and_repeated_values(vec![
                vec![1, 2],
                vec![1, 3],
            ]),
            vec![1, 4]
        );
    }
}