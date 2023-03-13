// LeetCode 26. Remove Duplicates from Sorted Array
impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        nums.dedup();
        return nums.len() as i32;
    }
}
