// LeetCode 242. Valid Anagram
use std::collections::HashMap;
impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() { return false; }

        let mut pairs: HashMap<char, i32> = HashMap::new();
        s.chars().for_each(|c| *pairs.entry(c).or_insert(0) += 1);
        t.chars().for_each(|c| *pairs.entry(c).or_insert(0) -= 1);

        return pairs.into_values().all(|v| v == 0);
    }
}
