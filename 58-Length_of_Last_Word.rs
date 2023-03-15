// LeetCode 58. Length of Last Word
impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        return s.clone().trim().split(" ").last().unwrap().len() as i32;
    }
}
