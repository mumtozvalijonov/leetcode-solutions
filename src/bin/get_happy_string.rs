// 1415. The k-th Lexicographical String of All Happy Strings of Length n

struct Solution;

fn main() {
    println!("{}", Solution::get_happy_string(1, 3));
}

impl Solution {
    pub fn get_happy_string(n: i32, k: i32) -> String {
        let mut k = k - 1;
        let mut n = n as u32;
        if k >= 3 * 2_i32.pow(n - 1) {
            return "".to_string();
        }
        let first_letter_arr: [char; 3] = ['a', 'b', 'c'];
        let a_arr: [char; 2] = ['b', 'c'];
        let b_arr: [char; 2] = ['a', 'c'];
        let c_arr: [char; 2] = ['a', 'b'];
        let mut result = String::new();

        let mut last_char_idx = k / 2_i32.pow(n - 1);
        let mut last_char = first_letter_arr[last_char_idx as usize];
        result.push(last_char);
        k %= 2_i32.pow(n - 1);
        n -= 1;

        while n > 0 {
            last_char_idx = k / 2_i32.pow(n - 1);
            last_char = match last_char {
                'a' => a_arr[last_char_idx as usize],
                'b' => b_arr[last_char_idx as usize],
                _ => c_arr[last_char_idx as usize],
            };
            result.push(last_char);
            k %= 2_i32.pow(n - 1);
            n -= 1;
        }
        result.chars().collect::<String>()
    }
}
