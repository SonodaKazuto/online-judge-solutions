// LeetCode 367. Valid Perfect Square
impl Solution {
    pub fn is_perfect_square(num: i32) -> bool {
        if num == 1 { return true; }

        for i in 2..=((num as f32).sqrt().ceil() as i32) {
            if (num % i == 0) & (i * i == num) { return true; } 
        }

        return false;
    }
}
