// LeetCode 338. Counting Bits
impl Solution {
    pub fn count_bits(n: i32) -> Vec<i32> {
        let mut bits: Vec<i32> = Vec::new();

        for i in 0..=n {
            let binary_form: String = format!("{:b}", i);
            bits.push(binary_form.chars().filter(|c| *c == '1').count() as i32);
        }

        return bits;
    }
}
