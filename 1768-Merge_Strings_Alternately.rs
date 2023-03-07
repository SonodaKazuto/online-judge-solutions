// LeetCode 1768. Merge Strings Alternately
impl Solution {
    pub fn merge_alternately(word1: String, word2: String) -> String {
        let mut vec_word1 = word1.chars().peekable();
        let mut vec_word2 = word2.chars().peekable();
        let mut merged_string: String = String::new();

        while (vec_word1.peek() != None) | (vec_word2.peek() != None) {
            if let Some(c) = vec_word1.next() { merged_string.push(c); }
            if let Some(c) = vec_word2.next() { merged_string.push(c); }
        }

        return merged_string;
    }
}
