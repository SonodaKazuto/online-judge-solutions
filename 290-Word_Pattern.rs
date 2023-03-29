// LeetCode 290. Word Pattern
use std::collections::HashMap;
impl Solution {
    pub fn word_pattern(pattern: String, s: String) -> bool {
        let words: Vec<&str> = s.split_whitespace().collect();
        let p: Vec<char> = pattern.chars().collect();
        if words.len() != p.len() { return false; }
        
        let mut dict: HashMap<char, String> = HashMap::new();
        
        for i in 0..words.len() {
            let inference = dict.entry(p[i]).or_insert(words[i].to_string());
            if *inference != words[i].to_string() { return false; }
            else { if dict.values().filter(|&v| *v == words[i].to_string()).count() > 1 { return false; } }
        }

        return true;
    }
}
