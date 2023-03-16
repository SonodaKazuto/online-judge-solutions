// LeetCode 125. Valid Palindrome
impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        if s.len() <= 1 { return true; }

        let mut test_string: String = s.replace(|c: char| (!c.is_alphabetic() & !c.is_numeric()), "").to_ascii_lowercase();
        let k: usize = test_string.len() / 2;

        let left: String = test_string[0..k].to_string();
        let mut right: String = test_string[k..].to_string().chars().rev().collect();
        if test_string.len() % 2 == 1 { right = test_string[(k + 1)..].to_string().chars().rev().collect(); }
        return left == right;
    }
}
