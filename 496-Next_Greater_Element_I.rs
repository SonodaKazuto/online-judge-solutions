// LeetCode 496. Next Greater Element I
impl Solution {
    pub fn next_greater_element(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut next_greater: Vec<i32> = Vec::new();

        for n in nums1.iter() {
            let position: usize = nums2.iter()
                                       .position(|x| x == n)
                                       .unwrap();
            let ng: i32 = *nums2[(position + 1)..].iter()
                                                  .find(|x| x > &n)
                                                  .unwrap_or(&(-1));
            next_greater.push(ng);
        }

        return next_greater;
    }
}
