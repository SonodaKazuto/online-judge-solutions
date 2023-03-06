// LeetCode 1672. Richest Customer Wealth
impl Solution {
    pub fn maximum_wealth(accounts: Vec<Vec<i32>>) -> i32 {
        return accounts.iter()
                       .map(|a| a.iter().sum())
                       .max()
                       .unwrap();
    }
}
