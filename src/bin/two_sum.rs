fn main() {
    let nums = vec![2, 7, 11, 15];
    let target = 18;
    let result = two_sum(nums, target);
    println!("{:?}", result);
}

fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map = std::collections::HashMap::new();

    for (i, &val) in nums.iter().enumerate() {
        let complement = target - val;
        if let Some(&j) = map.get(&complement) {
            return vec![j as i32, i as i32];
        }
        map.insert(val, i as u16);
    }

    vec![]
}
