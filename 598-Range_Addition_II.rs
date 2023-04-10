// LeetCode 598. Range Addition II
use std::cmp::min;
impl Solution {
    pub fn max_count(m: i32, n: i32, ops: Vec<Vec<i32>>) -> i32 {
        if ops.is_empty() { return m * n; }

        let mut m_length: i32 = i32::max_value();
        let mut n_length: i32 = i32::max_value();
        for op in ops.into_iter() {
            m_length = min(op[0], m_length);
            n_length = min(op[1], n_length);
        }

        return m_length * n_length;
    }
}
