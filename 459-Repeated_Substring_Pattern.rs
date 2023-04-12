// LeetCode 459. Repeated Substring Pattern
impl Solution {
    pub fn repeated_substring_pattern(s: String) -> bool {
        if s.is_empty() | (s.len() == 1) { return false; }

        let string_length: i32 = s.len() as i32;
        let mut pattern: &str = "";
        for i in 1..string_length {
            if (string_length % i) == 0 { pattern = &s[0..(i as usize)]; }
            let mut repeated_string: String = String::new();
            for i in 0..(string_length / i) { repeated_string += pattern; }
            if s == repeated_string { return true; }
        }

        return false;
    }
}
