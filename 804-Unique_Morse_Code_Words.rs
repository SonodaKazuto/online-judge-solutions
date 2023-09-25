use std::collections::HashSet;

impl Solution {
    pub fn unique_morse_representations(words: Vec<String>) -> i32 {
        return words.into_iter()
                    .map(|w| Solution::morse_conversion(w))
                    .collect::<HashSet<_>>()
                    .into_iter()
                    .count() as i32;
    }

    pub fn morse_conversion(word: String) -> String {
        let morse_codes: Vec<&str> = vec!(".-", "-...", "-.-.", "-..", ".", 
                                          "..-.", "--.", "....", "..", ".---", 
                                          "-.-", ".-..", "--", "-.", "---", 
                                          ".--.", "--.-", ".-.", "...", "-", 
                                          "..-", "...-", ".--", "-..-", "-.--", 
                                          "--..");

        let mut encoded_string: String = String::new();
        for c in word.to_ascii_lowercase().chars() {
            encoded_string += morse_codes.get(((c as u32) - 97) as usize).unwrap();
        }

        return encoded_string;
    }
}							
