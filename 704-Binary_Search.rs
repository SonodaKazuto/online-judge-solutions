// LeetCode 704. Binary Search
use std::cmp::Ordering;

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        if nums.is_empty() { return -1; }

        let (mut low, mut high) = (0, nums.len());

        while low < high {
            let mid = (low + high) / 2;
            match target.cmp(&nums[mid]) {
                Ordering::Equal => return mid as i32,
                Ordering::Less => high = mid,
                Ordering::Greater => low = mid + 1,
            }
        }

        return -1;
    }
}
