impl Solution {
    pub fn num_jewels_in_stones(jewels: String, stones: String) -> i32 {
        let stones: Vec<char> = stones.chars().collect();
        let mut number_of_jewels: usize = 0;

        for jewel in jewels.chars() { 
            number_of_jewels += stones.iter()
                                      .filter_map(|&stone| if stone == jewel { Some(stone) } else { None })
                                      .collect::<Vec<char>>()
                                      .iter()
                                      .count(); 
        }

        return number_of_jewels as i32;
    }
}
