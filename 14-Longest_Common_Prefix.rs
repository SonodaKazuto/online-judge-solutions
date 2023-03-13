// LeetCode 14. Longest Common Prefix
impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut prefix: String = String::new();

        if strs.len() == 1 { return strs[0].clone(); }
        let mut str1 = strs[0].chars().peekable();
        let mut str2 = strs[1].chars().peekable();

        loop {
            if (str1.peek() == None) | (str2.peek() == None) { break; }
            if (str1.peek() == str2.peek()) { prefix += &str1.peek().unwrap().to_string(); }
            else { break; }
            str1.next();
            str2.next();
        }

        if strs.len() == 2 { return prefix; }

        for i in 2..strs.len() {
            let tmp: String = prefix.clone();
            str1 = tmp.chars().peekable();
            str2 = strs[i].chars().peekable();

            prefix = String::new();
            loop {
                if (str1.peek() == None) | (str2.peek() == None) { break; }
                if (str1.peek() == str2.peek()) { prefix += &str1.peek().unwrap().to_string(); }
                else { break; }
                str1.next();
                str2.next();
            }
        }

        return prefix;
    }
}
