// LeetCode 168. Excel Sheet Column Title
impl Solution {
    pub fn convert_to_title(column_number: i32) -> String {
        let mut column_num: i32 = column_number - 1;
        let mut column: Vec<u8> = Vec::new();

        while column_num >= 0 {
            column.push((65 + column_num % 26) as u8);
            column_num = column_num / 26 - 1;
        }

        column.reverse();
        return String::from_utf8(column).unwrap();
    }
}
