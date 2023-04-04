// LeetCode 521. Longest Uncommon Subsequence I
impl Solution {
    pub fn find_lu_slength(a: String, b: String) -> i32 {
        return match a == b {
            true => -1,
            false => (a.len()).max(b.len()) as i32
        }
    }
}
