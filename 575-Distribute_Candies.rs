// LeetCode 575. Distribute Candies
use std::collections::HashSet;
use std::iter::FromIterator;
impl Solution {
    pub fn distribute_candies(candies: Vec<i32>) -> i32 {
        if candies.is_empty() { return 0; }

        let candy_type: HashSet<i32> = HashSet::from_iter(candies.clone());
        return match candy_type.len() <= (candies.len() / 2) {
            true => candy_type.len(),
            false => candies.len() / 2
        } as i32;
    }
}
