// LeetCode 136. Single Number
impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut numbers: Vec<i32> = nums.clone();
        numbers.sort();

        for i in 0..(numbers.len() - 1) {
            if (i % 2) == 0 {
                if numbers[i] != numbers[i + 1] { return numbers[i]; }
            }
        }

        return *numbers.last().unwrap();
    }
}
