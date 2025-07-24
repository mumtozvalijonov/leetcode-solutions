// 1071. Greatest Common Divisor of Strings
struct Solution;

fn main() {
    println!(
        "{}",
        Solution::gcd_of_strings("ABCABC".to_owned(), "ABC".to_owned())
    );
    // let a = 36;
    // let b = 48;
    // for d in Solution::common_divisors(a, b) {
    //     println!("{}", d);
    // }
}

impl Solution {
    fn _gcd(a: usize, b: usize) -> usize {
        if b == 0 {
            a
        } else {
            Self::_gcd(b, a % b)
        }
    }

    fn common_divisors(a: usize, b: usize) -> impl Iterator<Item = usize> {
        let g = Self::_gcd(a, b);
        let mut current = g;
        std::iter::from_fn(move || {
            while current >= 1 {
                let candidate = current;
                current -= 1;
                if g % candidate == 0 {
                    return Some(candidate);
                }
            }
            None
        })
    }

    pub fn gcd_of_strings(str1: String, str2: String) -> String {
        let mut result = String::new();
        let (str1, str2) = if str1.len() >= str2.len() {
            (str1, str2)
        } else {
            (str2, str1)
        };
        if str1[..str2.len()] != str2 {
            return "".to_owned();
        }
        for div in Self::common_divisors(str1.len(), str2.len()) {
            let (str1_div, str2_div) = (&str1[..div], &str2[..div]);
            if str1_div != str2_div {
                continue;
            }
            if str1_div.repeat(str1.len() / div) == str1
                && str2_div.repeat(str2.len() / div) == str2
            {
                result = str1_div.to_owned();
                break;
            }
        }
        result
    }
}
