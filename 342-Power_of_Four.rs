// LeetCode 342. Power of Four
impl Solution {
    pub fn is_power_of_four(n: i32) -> bool {
        if (n == 1) | (n == 4) { return true; }
        if (n < 1) | (n % 2 == 1) { return false; }

        let mut number: i32 = n;
        while number % 4 == 0 { number /= 4; }

        return number == 1;
    }
}
