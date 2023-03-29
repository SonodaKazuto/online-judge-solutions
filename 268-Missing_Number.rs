// LeetCode 268. Missing Number
impl Solution {
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        let sum_range: i32 = (0..=(nums.len() as i32)).sum();
        let sum_numbers: i32 = nums.iter().sum();
        return sum_range - sum_numbers;
    }
}
