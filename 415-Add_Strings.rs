// LeetCode 415. Add Strings
impl Solution {
    pub fn add_strings(num1: String, num2: String) -> String {
        let mut number1 = num1.chars().rev();
        let mut number2 = num2.chars().rev();
        let mut sum_numbers: String = String::new();
        let mut carry: i32 = 0;
        loop {
            let digit1 = number1.next();
            let digit2 = number2.next();

            if digit1.is_none() & digit2.is_none() { break; }

            if digit1.is_some() {
                carry += ((digit1.unwrap() as u32) - ('0' as u32)) as i32;
            }
            if digit2.is_some() {
                carry += ((digit2.unwrap() as u32) - ('0' as u32)) as i32;
            }
            sum_numbers = (carry % 10).to_string() + &sum_numbers;
            carry /= 10;
        }

        if carry != 0 { sum_numbers = carry.to_string() + &sum_numbers; }

        return sum_numbers;
    }
}
