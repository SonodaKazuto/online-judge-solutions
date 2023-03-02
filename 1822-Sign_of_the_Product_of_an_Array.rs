// LeetCode 1822. Sign of the Product of an Array
impl Solution {
    pub fn array_sign(nums: Vec<i32>) -> i32 {
        let mut negative_count: i32 = 0;

        for n in nums.iter() {
            if *n == 0 { return 0; }
            if *n < 0 { negative_count += 1; }
        }

        if negative_count % 2 == 1 { return -1; }
        return 1;
    }
}
