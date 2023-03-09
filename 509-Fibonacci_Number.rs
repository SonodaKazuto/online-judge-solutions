// LeetCode 509. Fibonacci Number
impl Solution {
    pub fn fib(n: i32) -> i32 {
        return match n {
            0 => 0,
            1 | 2 => 1,
            _ => Solution::fib(n - 1) + Solution::fib(n - 2)
        }
    }
}
