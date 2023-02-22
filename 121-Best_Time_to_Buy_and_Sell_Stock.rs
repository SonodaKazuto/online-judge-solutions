// LeetCode 121. Best Time to Buy and Sell Stock
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let num_days: usize = prices.len();
        let mut max_profit: i32 = 0;
        let mut min_price: i32 = 10000;

        if num_days < 2 { return 0; }

        for f in 0..(num_days - 1) {
            if prices[f] >= min_price { continue; }
            min_price = prices[f];
            for b in ((f + 1)..num_days).rev() {
                if (prices[b] - min_price) > max_profit {
                    max_profit = prices[b] - min_price;
                }
            }
        }

        return max_profit;
    }
}
      
