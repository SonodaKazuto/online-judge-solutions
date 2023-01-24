// LeetCode 392. Is Subsequence
use std::collections::HashMap;
impl Solution {
    pub fn is_isomorphic(s: String, t: String) -> bool {
        let s_chars: Vec<char> = s.chars().collect();
        let t_chars: Vec<char> = t.chars().collect();
        let mut occurance = HashMap::new();

        if s_chars.len() != t_chars.len() {
            return false;
        }
        for i in 0..s_chars.len() {
            if occurance.contains_key(s_chars.get(i).unwrap()) {
                if occurance.get(s_chars.get(i).unwrap()).unwrap() != &t_chars.get(i).unwrap() {
                    return false;
                }
            } else {
                occurance.insert(s_chars.get(i).unwrap(), t_chars.get(i).unwrap());
            }
        }
        occurance.clear();
        for i in 0..t_chars.len() {
            if occurance.contains_key(t_chars.get(i).unwrap()) {
                if occurance.get(t_chars.get(i).unwrap()).unwrap() != &s_chars.get(i).unwrap() {
                    return false;
                }
            } else {
                occurance.insert(t_chars.get(i).unwrap(), s_chars.get(i).unwrap());
            }
        }

        return true;
    }
}
