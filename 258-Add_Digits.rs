// LeetCode 258. Add Digits
impl Solution {
    pub fn add_digits(num: i32) -> i32 {
        if num < 10 { return num; }

        let mut number: i32 = num;
        let mut digit_sum: i32 = 0;
        loop {
            while number != 0 {
                digit_sum += number % 10;
                number /= 10;
            }
            if digit_sum < 10 { break; }
            number = digit_sum;
            digit_sum = 0;
        }

        return digit_sum;
    }
}
