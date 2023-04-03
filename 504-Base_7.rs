// LeetCode 504. Base 7
impl Solution {
    pub fn convert_to_base7(num: i32) -> String {
        if num == 0 { return "0".to_string(); }

        let mut seven_based_representation: String = String::new();
        let mut number: i32 = num.abs();
        while number > 0 {
            seven_based_representation = (number % 7).to_string() + &seven_based_representation;
            number /= 7;
        }

        if num < 0 { seven_based_representation = "-".to_string() + &seven_based_representation }
        return seven_based_representation;
    }
}
