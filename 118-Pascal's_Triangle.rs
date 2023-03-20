// LeetCode 118. Pascal's Triangle
impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let mut pascal_triangle: Vec<Vec<i32>> = Vec::new();

        for i in 1..((num_rows + 1) as usize) {
            match i {
                1 => pascal_triangle.push(vec![1]),
                2 => pascal_triangle.push(vec![1, 1]),
                _ => {
                    let mut last_layer: Vec<i32> = vec![1];
                    for w in pascal_triangle.last().unwrap().windows(2) { last_layer.push(w[0] + w[1]); }
                    last_layer.push(1);
                    pascal_triangle.push(last_layer);
                }
            }
        }

        return pascal_triangle;
    }
}
