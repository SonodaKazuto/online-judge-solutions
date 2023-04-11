// LeetCode 414. Third Maximum Number
impl Solution {
    pub fn third_max(nums: Vec<i32>) -> i32 {
        if nums.is_empty() { return i32::MIN; }

        let mut numbers: Vec<i32> = nums.clone();
        numbers.sort_unstable();
        numbers.dedup();
        numbers.reverse();
        if numbers.len() < 3 { return numbers[0]; }
        return numbers[2];
    }
}
