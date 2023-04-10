// LeetCode 561. Array Partition
impl Solution {
    pub fn array_pair_sum(nums: Vec<i32>) -> i32 {
        let mut numbers: Vec<i32> = nums;
        numbers.sort_unstable();
        return numbers.iter().step_by(2).sum();
    }
}
