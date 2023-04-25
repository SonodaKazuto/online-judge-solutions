// LeetCode 657. Robot Return to Origin
impl Solution {
    pub fn judge_circle(moves: String) -> bool {
        if moves.is_empty() { return true; }

        let mut coordinate: (i32, i32) = (0, 0);
        for c in moves.chars() {
            match c {
                'U' => coordinate.1 += 1,
                'D' => coordinate.1 -= 1,
                'L' => coordinate.0 -= 1,
                'R' => coordinate.0 += 1,
                _ => return false
            };
        }
        return coordinate == (0, 0);
    }
}
