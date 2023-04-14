// LeetCode 383. Ransom Note
use std::collections::HashMap;
impl Solution {
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        let mut character_count: HashMap<char, i32> = HashMap::new();
        for c in magazine.chars() { *character_count.entry(c).or_insert(0) += 1; }
        for c in ransom_note.chars() {
            if !character_count.contains_key(&c) { return false; }
            character_count.entry(c).and_modify(|count| *count -= 1);
        }

        return character_count.iter().all(|(_, &v)| v >= 0);
    }
}
