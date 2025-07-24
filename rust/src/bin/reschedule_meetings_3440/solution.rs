pub struct Solution;

impl Solution {
    pub fn max_free_time(event_time: i32, start_time: Vec<i32>, end_time: Vec<i32>) -> i32 {
        let n = start_time.len();
        let mut max_free_time = 0;
        for i in 0..n {
            let duration = end_time[i] - start_time[i];
            if i < n - 1 {
                max_free_time = max_free_time.max(start_time[i + 1] - end_time[i]);
            } else {
                max_free_time = max_free_time.max(event_time - end_time[i]);
            }
            for j in 0..n {
                if i == j {
                    continue;
                }
                let new_start_time = start_time[j] - duration;
                if (j > 0 && new_start_time < end_time[j - 1]) || (new_start_time < 0) {
                    continue;
                }
                let free_time_start = if i == 0 { 1 } else { end_time[i - 1] };  // fix here
                let possible_free_time = new_start_time - free_time_start;
                max_free_time = max_free_time.max(possible_free_time);
                println!(
                    "Moving meeting #{}: {} - {} = {}",
                    i, new_start_time, free_time_start, possible_free_time
                )
            }
            let new_start_time = event_time - duration;
            if new_start_time >= end_time[n - 1] {
                let free_time_start = end_time[n - 1];
                let possible_free_time = new_start_time - free_time_start;
                max_free_time = max_free_time.max(possible_free_time);
                println!(
                    "Moving meeting #{}: {} - {} = {}",
                    i, new_start_time, free_time_start, possible_free_time
                )
            }
        }
        max_free_time
    }
}
