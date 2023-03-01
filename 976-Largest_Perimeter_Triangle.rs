// LeetCode 976. Largest Perimeter Triangle
impl Solution {
    pub fn largest_perimeter(nums: Vec<i32>) -> i32 {
        let mut sides: Vec<i32> = nums.clone();

        sides.sort_unstable_by_key(|n| -n);
        
        return sides.windows(3).find(|w| w[0] < w[1] + w[2])
                               .map(|w| w[0] + w[1] + w[2])
                               .unwrap_or(0);
    }
}
