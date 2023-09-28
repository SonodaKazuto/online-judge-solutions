impl Solution {
    pub fn binary_gap(n: i32) -> i32 {
        return format!("{:b}", n).chars()
                                 .enumerate()
                                 .filter_map(|(i, c)| if c == '1' { Some(i) } else { None })
                                 .collect::<Vec<usize>>()
                                 .windows(2)
                                 .map(|w| w[1] - w[0])
                                 .max()
                                 .unwrap_or(0) as i32;
    }
}
