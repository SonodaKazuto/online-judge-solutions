// LeetCode 350. Intersection of Two Arrays II
use std::cmp;
impl Solution {
    pub fn intersect(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut intersection: Vec<i32> = Vec::new();
        for n in nums1.clone().into_iter() {
            if nums2.contains(&n) {
                let count: i32 = cmp::min(nums1.iter().filter(|&&x| x == n).count(), 
                                          nums2.iter().filter(|&&x| x == n).count()) as i32;
                if !intersection.contains(&n) { for i in 0..count { intersection.push(n); } }
            }
        }

        return intersection;
    }
}
