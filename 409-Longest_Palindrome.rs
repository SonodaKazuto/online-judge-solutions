// LeetCode 409. Longest Palindrome
use std::collections::HashMap;

impl Solution {
    pub fn longest_palindrome(s: String) -> i32 {
        let mut max_length: i32 = s.len() as i32;
        let mut character_count: HashMap<char, i32> = HashMap::new();


        if (s.len() == 0) | (s.len() == 1) { return s.len() as i32; }
        for c in s.chars() {
            if character_count.contains_key(&c) {
                character_count.insert(c, character_count.get(&c).unwrap() + 1);
            } else {
                character_count.insert(c, 1);
            }
        }

        let odd_count: i32 = character_count.values().filter(|v| **v % 2 == 1).count() as i32;

        return max_length - odd_count + (odd_count != 0) as i32;
    }
}
          
