// LeetCode 263. Ugly Number
impl Solution {
    pub fn is_ugly(n: i32) -> bool {
        if (n == 1) | (n == 2) | (n == 3) | (n == 5) { return true; }
        if n == 0 { return false; }

        let mut number: i32 = n;
        for f in vec![2, 3, 5] {
            while number % f == 0 {
                number /= f;
            }
        }

        return number == 1;
    }
}
