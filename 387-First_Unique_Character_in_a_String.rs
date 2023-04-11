// LeetCode 387. First Unique Character in a String
use std::collections::HashMap;
impl Solution {
    pub fn first_uniq_char(s: String) -> i32 {
        let mut frequency_list: HashMap<char, i32> = HashMap::new();
        for c in s.chars() { *frequency_list.entry(c).or_insert(0) += 1; }
        for (i, c) in s.chars().enumerate() { if frequency_list[&c] == 1 { return i as i32; }}

        return -1;
    }
}
