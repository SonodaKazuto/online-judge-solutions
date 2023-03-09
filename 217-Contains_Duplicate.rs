// LeetCode 217. Contains Duplicate
impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut dedup_list: Vec<i32> = nums.clone();
        dedup_list.sort_unstable();
        dedup_list.dedup();
        return dedup_list.len() != nums.len();
    }
}
