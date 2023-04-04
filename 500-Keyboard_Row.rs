// LeetCode 500. Keyboard Row
impl Solution {
    pub fn find_words(words: Vec<String>) -> Vec<String> {
        let mut checked_words: Vec<String> = Vec::new();
        for word in words {
            let mut rear_row: i32 = 0;
            let mut cur_row: i32 = 0;
            let mut row_signal: bool = true;
            for c in word.to_ascii_lowercase().chars() {
                cur_row = match c {
                    'q' | 'w' | 'e' | 'r' | 't' | 'y' | 'u' | 'i' | 'o' | 'p' => 1,
                    'a' | 's' | 'd' | 'f' | 'g' | 'h' | 'j' | 'k' | 'l' => 2,
                    'z' | 'x' | 'c' | 'v' | 'b' | 'n' | 'm' => 3,
                    _ => 4
                };
                if rear_row == 0 { rear_row = cur_row; }
                if cur_row != rear_row {
                    row_signal = false;
                    break;
                }
            }
            if row_signal == true { checked_words.push(word); }
        }

        return checked_words;
    }
}
