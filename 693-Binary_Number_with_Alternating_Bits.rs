// LeetCode 693. Binary Number with Alternating Bits
impl Solution {
    pub fn has_alternating_bits(n: i32) -> bool {
        let binary_format: Vec<char> = format!("{:b}", n).chars().collect::<Vec<char>>();
        for i in 0..(binary_format.len() - 1) {
            if binary_format[i] == binary_format[i + 1] { return false; }
        }
        return true;
    }
}
