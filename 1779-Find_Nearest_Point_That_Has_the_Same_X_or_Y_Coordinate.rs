// LeetCode 1779. Find Nearest Point That Has the Same X or Y Coordinate
impl Solution {
    pub fn nearest_valid_point(x: i32, y: i32, points: Vec<Vec<i32>>) -> i32 {
        return points.iter()
                     .enumerate()
                     .filter(|(_, point)| point[0] == x || point[1] == y)
                     .min_by_key(|(_, point)| (point[0] - x).abs() + (point[1] - y).abs())
                     .map(|(index, _)| index as i32)
                     .unwrap_or(-1);
    }
}
