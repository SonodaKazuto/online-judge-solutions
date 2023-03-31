// LeetCode 482. License Key Formatting
impl Solution {
    pub fn license_key_formatting(s: String, k: i32) -> String {
        let key: String = s.to_ascii_uppercase().replace("-", "");
        let mut formated_key: String = String::new();
        for (i, c) in key.chars().rev().enumerate() {
            if (i > 0) & (i % (k as usize) == 0) { formated_key.push('-'); } 
            formated_key.push(c);
        }

        return formated_key.chars().rev().collect();
    }
}
