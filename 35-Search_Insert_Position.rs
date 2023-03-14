// LeetCode 35. Search Insert Position
impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        return match nums.iter().position(|&x| x == target) {
            Some(i) => i as i32,
            _ => nums.into_iter()
                     .enumerate()
                     .filter(|&(_, x)| x < target)
                     .map(|(i, _)| i + 1)
                     .max()
                     .unwrap_or(0) as i32
        }
    }
}
