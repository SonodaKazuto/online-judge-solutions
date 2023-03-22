// LeetCode 190. Reverse Bits
impl Solution {
    pub fn reverse_bits(x: u32) -> u32 {
        let mut binary: String = format!("{:b}", x);
        
        for _ in 0..(32 - binary.len()) { binary.insert(0, '0'); }

        let mut decimal: u32 = 0;
        for (i, c) in binary.to_string().chars().enumerate() {
            if c == '1' { decimal += u32::pow(2, i as u32); }
        }

        return decimal;
    }
}
