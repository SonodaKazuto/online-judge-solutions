// LeetCode 349. Intersection of Two Arrays
use std::collections::HashSet;
impl Solution {
    pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut intersection: HashSet<i32> = HashSet::new();
        for n in nums1.into_iter() {
            if nums2.contains(&n) { intersection.insert(n); }
        }

        return intersection.into_iter().collect::<Vec<i32>>();
    }
}
