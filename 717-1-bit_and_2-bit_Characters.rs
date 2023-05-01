// LeetCode 717. 1-bit and 2-bit Characters
use std::collections::VecDeque;
impl Solution {
    pub fn is_one_bit_character(bits: Vec<i32>) -> bool {
        if bits.is_empty() { return false; }

        let mut bits: VecDeque<i32> = bits.into_iter().collect::<VecDeque<i32>>();
        loop {
            if bits.len() <= 2 { break; }
            let first: i32 = *bits.front().unwrap();
            bits.pop_front();
            if first == 1 { bits.pop_front(); }
        }

        return bits.iter().filter(|&x| *x == 0).count() == bits.len();
    }
}
