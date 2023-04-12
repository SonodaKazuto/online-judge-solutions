// LeetCode 434. Number of Segments in a String
impl Solution {
    pub fn count_segments(s: String) -> i32 {
        let mut trimmed_string: String = s.trim().to_string();
        if trimmed_string.is_empty() { return 0; }

        return trimmed_string.split_whitespace().collect::<Vec<&str>>().iter().count() as i32;
    }
}
