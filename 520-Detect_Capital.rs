// LeetCode 520. Detect Capital
impl Solution {
    pub fn detect_capital_use(word: String) -> bool {
        if word == word.to_ascii_uppercase() { return true; }
        if word == word.to_ascii_lowercase() { return true; }
        if word == ((&word[0..1].to_string()).to_ascii_uppercase() + &(&word[1..].to_string().to_ascii_lowercase())) {
            return true;
        }

        return false
    }
}
