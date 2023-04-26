// LeetCode 448. Find All Numbers Disappeared in an Array
use std::collections::HashMap;
impl Solution {
    pub fn find_disappeared_numbers(nums: Vec<i32>) -> Vec<i32> {
        let length: usize = nums.len();
        let mut hit_table: HashMap<i32, bool> = HashMap::with_capacity(length);
        for i in 1..=length { hit_table.entry(i as i32).or_insert(false); }

        for n in nums.into_iter() { hit_table.entry(n).and_modify(|v| { *v = true; }); }
        
        return hit_table.into_iter()
                        .filter_map(|(k, v)| if (!v) { Some(k) } else { None })
                        .collect::<Vec<i32>>();
    }
}
