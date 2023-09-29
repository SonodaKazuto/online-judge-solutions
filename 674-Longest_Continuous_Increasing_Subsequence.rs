impl Solution {
    pub fn find_length_of_lcis(nums: Vec<i32>) -> i32 {
        let comparison: Vec<i32> = nums.windows(2)
                                       .map(|w| (w[1] > w[0]) as i32)
                                       .collect();
        if comparison.iter().all(|e| *e == 0) { return 1; }
        if comparison.iter().all(|e| *e == 1) { return nums.len() as i32; }

        let (mut longest, mut count) = (0, 0);
        for c in comparison.into_iter() {
            if c == 0 { count = 0; }
            else { count += 1; }
            if count > longest { longest = count; }
        }

        return longest + 1;
    }
}
