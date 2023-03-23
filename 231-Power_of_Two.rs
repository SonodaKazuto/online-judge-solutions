// LeetCode 231. Power of Two
impl Solution {
    pub fn is_power_of_two(n: i32) -> bool {
        if n == 1 { return true; }
        if (n == 0) | (n % 2 == 1) { return false; }

        return (n as f64).log2().abs() % 1.0 == 0.0;
    }
}
