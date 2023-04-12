// LeetCode 441. Arranging Coins
impl Solution {
    pub fn arrange_coins(n: i32) -> i32 {
        if n == 0 { return -1; }

        let mut num_of_stair: i32 = 1;
        let mut total_coins: i32 = n;
        loop {
            total_coins -= num_of_stair;
            if total_coins < 0 { return num_of_stair - 1; }
            if total_coins == 0 { return num_of_stair; }
            num_of_stair += 1;
        }
        
        return -1;
    }
}
