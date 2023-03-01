// LeetCode 191. Number of 1 Bits
impl Solution {
    pub fn hammingWeight (n: u32) -> i32 {
        let mut binary: u32 = n;
        let mut count: i32 = 0;

        for _ in 0..32 {
            count += (binary & 1) as i32;
            binary >>= 1;
        }

        return count;
    }
}
