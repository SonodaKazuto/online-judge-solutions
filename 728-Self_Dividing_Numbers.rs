// LeetCode 728. Self Dividing Numbers
impl Solution {
    pub fn is_self_divided(number: i32) -> bool {
        let mut digits: Vec<i32> = number.to_string()
                                         .chars()
                                         .map(|d| d.to_digit(10)
                                                   .unwrap() as i32)
                                         .collect::<Vec<i32>>();

        if digits.contains(&0) { return false; }
        return digits.iter().all(|n| (number % n) == 0);
    }

    pub fn self_dividing_numbers(left: i32, right: i32) -> Vec<i32> {
        let mut self_dividings: Vec<i32> = Vec::new();
        for n in left..=right {
            if Self::is_self_divided(n) { self_dividings.push(n); }
        }
        return self_dividings;
    }
}
