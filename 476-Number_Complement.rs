// LeetCode 476. Number Complement
impl Solution {
    pub fn find_complement(num: i32) -> i32 {
        let binary: String = format!("{:b}", num);
        let mut complement: i32 = 0;
        for b in binary.chars() {
            complement *= 2;
            if b == '0' { complement += 1; }
        }

        return complement;
    }
}
