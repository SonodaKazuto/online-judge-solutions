// LeetCode 88. Merge Sorted Array
impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        nums1.resize((m as usize), 0);
        nums2.resize((n as usize), 0);
        nums1.append(nums2);
        nums1.sort();
    }
}
