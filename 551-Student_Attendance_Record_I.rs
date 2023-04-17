// LeetCode 551. Student Attendance Record I
impl Solution {
    pub fn check_record(s: String) -> bool {
        return (s.chars().filter(|&c| c == 'A').count() < 2) && !s.contains("LLL");
    }
}
