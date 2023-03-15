// LeetCode 70. Climbing Stairs
impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        if n <= 3 { return n; }
        
        let mut n1: i32 = 2;
        let mut n2: i32 = 3;

        for _ in 0..(n - 3) {
            let tmp: i32 = n1;
            n1 = n2;
            n2 = n1 + tmp;
        }

        return n2;
    }
}
