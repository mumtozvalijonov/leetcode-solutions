// 2342. Max Sum of a Pair With Equal Sum of Digits


fn main() {
    let nums: Vec<i32> = vec![18,43,36,13,7];
    let max_sum = maximum_sum(nums);
    print!("{}", max_sum);
}

fn maximum_sum(nums: Vec<i32>) -> i32 {
    let mut max_by_ds = [0; 100];
    let mut max_sum = -1;
    
    for value in nums.iter() {
        let mut num = *value;
        let mut sum_digits = 0_u8;
        while num > 0 {
            sum_digits += (num%10) as u8;
            num /= 10;
        }

        let y = max_by_ds[sum_digits as usize];
        if y > 0 {
            max_sum = std::cmp::max(max_sum, y + *value);
        }
        max_by_ds[sum_digits as usize] = std::cmp::max(y, *value);
    }

    max_sum
}
