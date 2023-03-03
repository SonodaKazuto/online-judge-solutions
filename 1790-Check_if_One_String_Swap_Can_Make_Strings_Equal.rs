// LeetCode 1790. Check if One String Swap Can Make Strings Equal
impl Solution {
    pub fn are_almost_equal(s1: String, s2: String) -> bool {
        if s1 == s2 { return true; }

        let string_1: Vec<char> = s1.chars().collect();
        let string_2: Vec<char> = s2.chars().collect();

        if string_1.len() != string_2.len() { return false; }

        let mut swap_pair: Vec<char> = Vec::new();
        let mut swap_flag: bool = false;
        let mut same_cnt: i32 = 0;

        for i in 0..string_1.len() {
            if string_1[i] != string_2[i] {
                if !swap_flag {
                    swap_pair.push(string_2[i]);
                    swap_pair.push(string_1[i]);
                    swap_flag = true;
                } else {
                    if (string_1[i] != swap_pair[0]) | (string_2[i] != swap_pair[1]) {
                        return false;
                    }
                }
            } else {
                same_cnt += 1;
            }
        }

        return (same_cnt + 2) == string_1.len() as i32;
    }
}
