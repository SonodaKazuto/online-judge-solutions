// LeetCode 389. Find the Difference
use std::collections::HashMap;
impl Solution {
    pub fn find_the_difference(s: String, t: String) -> char {
        if s.is_empty() { return t.chars().nth(0).unwrap(); }

        let mut frequency: HashMap<char, i32> = HashMap::new();
        for c in t.chars() { *frequency.entry(c).or_insert(0) += 1; }
        for c in s.chars() { frequency.entry(c).and_modify(|v| { *v -= 1 }); }

        return *frequency.into_iter()
                         .filter_map(|(k, v)| if (v != 0) { Some(k) } else { None })
                         .collect::<Vec<char>>()
                         .first()
                         .unwrap();
    }
}
