// LeetCode 326. Power of Three
impl Solution {
    pub fn is_power_of_three(n: i32) -> bool {
        if n < 1 { return false; }

        let mut number: i32 = n;
        while number % 3 == 0 { number /= 3; }

        return number == 1;
    }
}
