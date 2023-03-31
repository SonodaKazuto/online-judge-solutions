// LeetCode 485. Max Consecutive Ones
impl Solution {
    pub fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
        let mut max_length: i32 = 0;
        let mut sub_seq_length: i32 = 0;
        for n in nums.into_iter() { 
            if n == 1 { sub_seq_length += 1; }
            else {
                if sub_seq_length > max_length { max_length = sub_seq_length; }
                sub_seq_length = 0;
            }
        }
        if sub_seq_length > max_length { max_length = sub_seq_length; }
        return max_length;
    }
}
