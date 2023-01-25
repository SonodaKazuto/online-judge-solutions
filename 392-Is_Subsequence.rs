// LeetCode 392. Is Subsequence
impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        let s_chars: Vec<char> = s.chars().collect();
        let mut pointer: i32 = 0;

        if s.is_empty() { return true; }
        for c in t.chars() {
            if &c == s_chars.get(pointer as usize).unwrap() {
                pointer = pointer + 1;
                if pointer == (s_chars.len() as i32) { break; }
            }
        }

        return pointer == (s_chars.len() as i32);
    }
}
