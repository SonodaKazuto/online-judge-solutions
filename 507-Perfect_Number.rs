// LeetCode 507. Perfect Number
use std::collections::HashSet;
impl Solution {
    pub fn check_perfect_number(num: i32) -> bool {
        if num == 1 { return false; }

        let mut divisors: HashSet<i32> = HashSet::from([1]);
        for i in 2..=(((num as f32).sqrt().ceil() as i32) + 1) {
            if num % i == 0 {
                divisors.insert(i);
                divisors.insert(num / i);
            }
        }

        return divisors.iter().sum::<i32>() == num;
    }
}
