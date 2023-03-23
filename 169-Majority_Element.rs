// LeetCode 169. Majority Element
impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut numbers: Vec<i32> = nums.clone();
        numbers.sort();
        numbers.dedup();
        for n in numbers.iter() {
            if nums.iter().filter(|x| x == &n).count() > (nums.len() / 2) {
                return *n;
            }
        }

        return 1000000001;
    }
}
