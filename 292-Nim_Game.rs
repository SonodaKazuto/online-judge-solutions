// LeetCode 292. Nim Game
impl Solution {
    pub fn can_win_nim(n: i32) -> bool {
        return (n % 4) != 0;
    }
}
