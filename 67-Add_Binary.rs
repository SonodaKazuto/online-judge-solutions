// LeetCode 67. Add Binary
impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        if a.is_empty() { return b; }
        if b.is_empty() { return a; }

        let mut b1 = a.chars().rev();
        let mut b2 = b.chars().rev();
        let mut result: String = String::new();
        let mut carry: i32 = 0;

        loop {
            match (b1.next(), b2.next()) {
                (Some('1'), Some('1')) => carry += 2,
                (Some('1'), _) | (_, Some('1')) => carry += 1,
                (Some('0'), _) | (_, Some('0')) => carry += 0,
                _ => break
            }
            result = (carry % 2).to_string() + &result;
            carry /= 2;
        }

        if carry != 0 { result = carry.to_string() + &result; }

        return result;
    }
}
