// LeetCode 1232. Check If It Is a Straight Line
impl Solution {
    pub fn check_straight_line(coordinates: Vec<Vec<i32>>) -> bool {
        for w in coordinates.windows(3) {
            if ((w[1][1] - w[0][1]) * (w[2][0] - w[1][0]) != (w[2][1] - w[1][1]) * (w[1][0] - w[0][0])) {
                return false;
            }
        }
        return true;
    }
}
