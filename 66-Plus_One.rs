// LeetCode 66. Plus One
impl Solution {
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        let mut number: Vec<i32> = Vec::new();
        let mut carry: i32 = 1;

        for n in digits.clone().into_iter().rev() {
            number.insert(0, ((carry + n) % 10));
            carry = (carry + n) / 10; 
        }

        if carry != 0 { number.insert(0, carry); }

        return number;
    }
}
