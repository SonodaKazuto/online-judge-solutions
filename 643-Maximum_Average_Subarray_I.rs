// LeetCode 643. Maximum Average Subarray I
impl Solution {
    pub fn find_max_average(nums: Vec<i32>, k: i32) -> f64 {
        if (k == 0) || nums.is_empty() { return 0 as f64; }
        return nums.windows(k as usize)
                   .map(|w| (w.iter().sum::<i32>() as f64) / (k as f64))
                   .max_by(|a, b| a.partial_cmp(b).unwrap())
                   .unwrap();
    }
}
