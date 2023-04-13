// LeetCode 463. Island Perimeter
impl Solution {
    pub fn island_perimeter(grid: Vec<Vec<i32>>) -> i32 {
        if grid.is_empty() { return 0; }

        let row: usize = grid.len();
        let column: usize = grid[0].len();
        let mut coast_length: i32 = 0;
        for x in 0..row {
            for y in 0..column {
                if grid[x][y] == 1 {
                    coast_length += 4;
                    if (x > 0) && (grid[x - 1][y] == 1) { coast_length -= 2; }
                    if (y > 0) && (grid[x][y - 1] == 1) { coast_length -= 2; }
                }
            }
        }

        return coast_length;
    }
}
