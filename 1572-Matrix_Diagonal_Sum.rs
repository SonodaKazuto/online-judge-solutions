// LeetCode 1572. Matrix Diagonal Sum
impl Solution {
    pub fn diagonal_sum(mat: Vec<Vec<i32>>) -> i32 {
        let mut summation: i32 = 0;
        let size: usize = mat.len();

        for i in 0..size {
            summation += mat[i][i];
            if i != (size - i - 1) { summation += mat[i][size - i - 1]; }
        }

        return summation;
    }
}
