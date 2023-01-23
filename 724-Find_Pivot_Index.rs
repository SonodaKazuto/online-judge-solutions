 // LeetCode 724. Find Pivot Index
 impl Solution {
    pub fn pivot_index(nums: Vec<i32>) -> i32 {
        let mut right_sum: i32 = nums.iter().sum();
        let mut left_sum: i32 = 0;

        for pivot in 0..nums.len() {
            right_sum = right_sum - nums.get(pivot).unwrap();
            if left_sum == right_sum {
                return pivot as i32;
            }
            left_sum = left_sum + nums.get(pivot).unwrap();
        }
        return -1;
    }
}
