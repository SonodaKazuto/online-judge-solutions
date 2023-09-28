impl Solution {
    pub fn sort_array_by_parity(nums: Vec<i32>) -> Vec<i32> {
        if nums.len() <= 1 { return nums; }

        let (mut even, mut odd): (Vec<_>, Vec<_>) = nums.into_iter()
                                                        .partition(|n| n % 2 == 0);
        even.append(&mut odd);
        return even;
    }
}
