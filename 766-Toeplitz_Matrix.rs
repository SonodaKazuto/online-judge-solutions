impl Solution {
    pub fn is_toeplitz_matrix(matrix: Vec<Vec<i32>>) -> bool {
        let m: usize = matrix.len();
        let n: usize = matrix[0].len();

        if (m == 1) || (n == 1) { return true; }

        for i in 0..(n - 1) {
            if matrix[1][i + 1] != matrix[0][i] { return false; }
        }

        for i in 0..(m - 1) {
            if matrix[i + 1][1] != matrix[i][0] { return false; }
        }

        let mut rest_matrix: Vec<Vec<i32>> = Vec::new();
        for i in 1..m { rest_matrix.push(matrix[i][1..].to_vec()); }
        return Solution::is_toeplitz_matrix(rest_matrix);
    }
}
