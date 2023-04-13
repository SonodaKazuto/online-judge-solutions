// LeetCode 495. Teemo Attacking
impl Solution {
    pub fn find_poisoned_duration(time_series: Vec<i32>, duration: i32) -> i32 {
        if time_series.is_empty() { return 0; }

        let mut timer: Vec<i32> = vec![0, 0];
        let mut attack_time: i32 = 0;
        for t in time_series.into_iter() {
            if (t > 0) && (timer[1] >= t) { attack_time += (t - timer[0]); }
            else { attack_time += duration; }
            timer = vec![t, (t + duration - 1)];
        }

        return attack_time;
    }
}
