// LeetCode 747. Largest Number At Least Twice of Others
impl Solution {
    pub fn dominant_index(nums: Vec<i32>) -> i32 {
        let mut numbers: Vec<i32> = nums.clone();
        numbers.sort_unstable();
        let largest_number: i32 = numbers.pop().unwrap();
        let second_largest_number: i32 = numbers.pop().unwrap();
        return match largest_number >= (second_largest_number * 2) {
            true => nums.into_iter().position(|x| x == largest_number).unwrap() as i32,
            false => -1
        };
    }
}
