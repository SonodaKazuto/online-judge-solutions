// LeetCode 762. Prime Number of Set Bits in Binary Representation
impl Solution {
    pub fn is_prime(number: i32) -> bool {
        if number == 2 { return true; }
        if (number <= 1) || ((number % 2) == 0) { return false; }

        for factor in 2..=((number as f32).sqrt().ceil() as i32) {
            if (number % factor) == 0 { return false; }
        }
        return true;
    }

    pub fn count_prime_set_bits(left: i32, right: i32) -> i32 {
        let mut count: i32 = 0;
        for number in left..=right {
            let count_set_bits: i32 = format!("{:b}", number).chars().filter(|&c| c == '1').count() as i32;
            count += Self::is_prime(count_set_bits) as i32;
        }
        return count;
    }
}
