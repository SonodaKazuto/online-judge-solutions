// LeetCode 1502. Can Make Arithmetic Progression From Sequence
impl Solution {
    pub fn can_make_arithmetic_progression(arr: Vec<i32>) -> bool {
        let mut sequence: Vec<i32> = arr.clone();

        sequence.sort();
        for w in sequence.windows(3) {
            if (w[1] - w[0]) != (w[2] - w[1]) {
                return false;
            }
        }

        return true;
    }
}
