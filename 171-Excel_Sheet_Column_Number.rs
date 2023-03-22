// LeetCode 171. Excel Sheet Column Number
impl Solution {
    pub fn title_to_number(column_title: String) -> i32 {
        let mut column_number: i32 = 0;

        for (i, c) in column_title.chars().rev().enumerate() {
            column_number += (c as u32 - 64) as i32 * i32::pow(26, i as u32);
        }

        return column_number;
    }
}
