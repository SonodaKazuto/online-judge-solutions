// LeetCode 1356. Sort Integers by The Number of 1 Bits
impl Solution {
    pub fn get_bits(number: i32) -> i32 {
        return format!("{:b}", number).chars()
                                      .filter(|c| *c == '1')
                                      .count() as i32;
    }

    pub fn sort_by_bits(arr: Vec<i32>) -> Vec<i32> {
        let mut sorted_array: Vec<i32> = arr.clone();
        sorted_array.sort_unstable_by(|n1, n2| {
            let n1_bits: i32 = Solution::get_bits(*n1);
            let n2_bits: i32 = Solution::get_bits(*n2);

            match n1_bits == n2_bits {
                true => n1.cmp(n2),
                false => n1_bits.cmp(&n2_bits)
            }
        });
        return sorted_array;
    }
}
