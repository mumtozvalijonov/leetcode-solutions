// 2579. Count Total Number of Colored Cells

struct Solution;

fn main() {
    println!("{}", Solution::colored_cells(1));
}

impl Solution {
    pub fn colored_cells(n: i32) -> i64 {
        let n = n as i64;
        let a1 = 1;
        let an = a1 + 2 * (n - 1);

        let s = (a1 + an) * (n - 1) + 1;
        s
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::colored_cells(1), 1);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::colored_cells(2), 5);
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::colored_cells(3), 13);
    }

    #[test]
    fn test_4() {
        assert_eq!(Solution::colored_cells(4), 25);
    }

    #[test]
    fn test_5() {
        assert_eq!(Solution::colored_cells(5), 41);
    }

    #[test]
    fn test_6() {
        assert_eq!(Solution::colored_cells(6), 61);
    }

    #[test]
    fn test_7() {
        assert_eq!(Solution::colored_cells(7), 85);
    }

    #[test]
    fn test_8() {
        assert_eq!(Solution::colored_cells(8), 113);
    }

    #[test]
    fn test_9() {
        assert_eq!(Solution::colored_cells(9), 145);
    }

    #[test]
    fn test_10() {
        assert_eq!(Solution::colored_cells(10), 181);
    }
}