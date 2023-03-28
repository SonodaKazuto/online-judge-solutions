// LeetCode 219. Contains Duplicate II
use std::collections::HashMap;
impl Solution {
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        if nums.is_empty() { return false; }

        let mut occured_numbers: HashMap<i32, usize> = HashMap::new();
        for (i, n) in nums.into_iter().enumerate() {
            let j = occured_numbers.entry(n).or_insert(i);
            if (*j != i) & (((*j - i) as i32).abs() <= k) { return true; } 
            else { *j = i; }
        }

        return false;
    }
}
