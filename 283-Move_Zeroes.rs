// LeetCode 283. Move Zeroes
impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut zeros: Vec<i32> = Vec::new();
        nums.retain(|x| if *x != 0 { true } else { zeros.push(0); false });
        &nums.append(&mut zeros);
    }
}
