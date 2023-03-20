// LeetCode 119. Pascal's Triangle II
impl Solution {
    pub fn get_row(row_index: i32) -> Vec<i32> {
        if row_index == 0 { return vec![1]; }
        if row_index == 1 { return vec![1, 1]; }

        let mut last_row: Vec<i32> = vec![1, 1];
        for i in 1..row_index {
            let mut next_row: Vec<i32> = vec![1];
            for w in last_row.windows(2) { next_row.push(w[0] + w[1]); };
            next_row.push(1);
            last_row = next_row;
        }

        return last_row;
    }
}
