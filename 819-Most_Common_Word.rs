use std::collections::HashMap;

impl Solution {
    pub fn most_common_word(paragraph: String, banned: Vec<String>) -> String {
        let mut words: Vec<String> = paragraph.replace("!", " ")
                                              .replace("?", " ")
                                              .replace("'", " ")
                                              .replace(",", " ")
                                              .replace(";", " ")
                                              .replace(".", " ")
                                              .to_ascii_lowercase()		
                                              .split_ascii_whitespace()
                                              .map(|s| s.to_string())
                                              .collect::<Vec<String>>();

        if !banned.is_empty() { banned.into_iter().for_each(|bw| words.retain(|w| *w != bw)); }
        
        let dict: HashMap<String, i32> = words.iter()
                                              .fold(HashMap::new(), |mut acc, c| {
                                                                    *acc.entry(c.to_string()).or_insert(0) += 1;
                                                                    acc});
        if dict.len() == 1 { return words.get(0).unwrap().to_string(); }
        
        return dict.into_iter().max_by_key(|e| e.1).unwrap().0;
    }
}
