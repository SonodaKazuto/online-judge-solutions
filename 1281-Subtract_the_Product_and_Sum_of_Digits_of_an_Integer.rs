// LeetCode 1281. Subtract the Product and Sum of Digits of an Integer
impl Solution {
    pub fn subtract_product_and_sum(n: i32) -> i32 {
        let mut number: i32 = n;
        let mut summation: i32 = 0;
        let mut production: i32 = 1;

        while number != 0 {
            summation += number % 10;
            production *= number % 10;

            number /= 10;
        }

        return production - summation;
    }
}
