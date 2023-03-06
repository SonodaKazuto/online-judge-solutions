// LeetCode 1588. Sum of All Odd Length Subarrays
impl Solution {
    pub fn sum_odd_length_subarrays(arr: Vec<i32>) -> i32 {
        let mut summation: i32 = 0;

        for n in (1..=arr.len()).step_by(2) {
            summation += arr.windows(n)
                            .map(|w| w.iter().sum::<i32>())
                            .sum::<i32>();
        }

        return summation;
    }
}
