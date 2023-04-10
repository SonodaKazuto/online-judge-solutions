// LeetCode 557. Reverse Words in a String III
impl Solution {
    pub fn reverse_words(s: String) -> String {
        let mut word: Vec<&str> = s.split(" ").collect::<Vec<&str>>();
        let mut reversed_string: String = String::new();
        for (i, w) in word.iter().enumerate() {
            reversed_string += &w.to_string().chars().rev().collect::<String>();
            if i != (word.len() - 1) { reversed_string += " "; }
        }

        return reversed_string;
    }
}
