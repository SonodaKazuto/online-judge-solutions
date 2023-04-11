// LeetCode 461. Hamming Distance
impl Solution {
    pub fn hamming_distance(x: i32, y: i32) -> i32 {
        return format!("{:b}", (x ^ y)).chars().filter(|c| c == &'1').count() as i32;
    }
}
