// LeetCode 599. Minimum Index Sum of Two Lists
impl Solution {
    pub fn find_restaurant(list1: Vec<String>, list2: Vec<String>) -> Vec<String> {
        let mut commons: Vec<String> = Vec::new(); 
        let mut index_sum: i32 = i32::MAX;
        for (i, word) in list1.into_iter().enumerate() {
            let word_index: Option<usize> = list2.iter().position(|w| w == &word);
            if word_index.is_some() {
                let sum: i32 = (word_index.unwrap() + i) as i32;
                if sum < index_sum { 
                    commons.clear();
                    index_sum = sum;
                }
                if sum == index_sum {
                    commons.push(word);
                }
            }
        }

        return commons;
    }
}
