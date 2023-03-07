// LeetCode 566. Reshape the Matrix
impl Solution {
    pub fn matrix_reshape(mat: Vec<Vec<i32>>, r: i32, c: i32) -> Vec<Vec<i32>> {
        if (r * c) != ((mat.len() * mat[0].len()) as i32) { return mat.clone(); }

        let mut reshaped_matrix: Vec<Vec<i32>> = Vec::new();
        let mut element_count: i32 = 0;
        for i in 0..mat.len() {
            for j in 0..mat[0].len() {
                if (element_count % c) == 0 { reshaped_matrix.push(Vec::new()); }
                reshaped_matrix[(element_count / c) as usize].push(mat[i][j]);
                element_count += 1;
            }
        }
    
        return reshaped_matrix;
    }
}
