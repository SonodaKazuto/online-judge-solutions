// LeetCode 1480. Running Sum of 1d Array
impl Solution {
    pub fn running_sum(nums: Vec<i32>) -> Vec<i32> {
        let mut summations = Vec::new();
        summations.push(0);
        for n in nums {
            summations.push(summations.last().unwrap() + n);
        }
        summations.remove(0);
        return summations;
    }
}
