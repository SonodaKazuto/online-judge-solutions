// LeetCode 28. Find the Index of the First Occurrence in a String
impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        return match haystack.find(&needle) {
            Some(i) => i as i32,
            _ => -1
        }
    }
}
