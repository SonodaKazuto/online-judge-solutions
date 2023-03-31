// LeetCode 492. Construct the Rectangle
impl Solution {
    pub fn construct_rectangle(area: i32) -> Vec<i32> {
        let mut rectangles: Vec<i32> = vec![1, area];
        for i in 2..=area {
            if (area % i == 0) & (i >= (area / i)) {
                if (i - (area / i)).abs() <= (rectangles[1] - rectangles[0]).abs() {
                    rectangles = vec![i, (area / i)];
                }
            }
        }
        return rectangles;
    }
}
