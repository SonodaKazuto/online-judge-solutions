// LeetCode 645. Set Mismatch
use std::collections::HashMap;
impl Solution {
    pub fn find_error_nums(nums: Vec<i32>) -> Vec<i32> {
        if nums.is_empty() { return vec![]; }

        let length: usize = nums.len();
        let mut hit_table: HashMap<i32, i32> = HashMap::with_capacity(length);
        for i in 1..=length { *hit_table.entry(i as i32).or_insert(0) += 1; }
        for n in nums.into_iter() { hit_table.entry(n).and_modify(|v| { *v -= 1; }); }

        let mut mismatch: Vec<i32> = vec![0, 0];
        for (k, v) in hit_table.into_iter() {
            if v == -1 { mismatch[0] = k; }
            if v == 1 { mismatch[1] = k; }
        }
        return mismatch;
    }
}
