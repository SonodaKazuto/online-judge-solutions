// LeetCode 202. Happy Number
impl Solution {
    pub fn p_sum(n: i32) -> i32 {
        let mut number: i32 = n;
        let mut p_sum: i32 = 0;

        while number != 0 {
            p_sum += (number % 10).pow(2);
            number /= 10;
        }

        return p_sum;
    }

    pub fn is_happy(n: i32) -> bool {
        let mut number: i32 = n;

        loop {
            if number / 10 == 0 {
                break;
            }
            number = Solution::p_sum(number);
        }

        return (number == 1) | (number == 7);
    }
}
