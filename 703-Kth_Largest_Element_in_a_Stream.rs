struct KthLargest {
    rank: Vec<i32>,
    order: usize
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl KthLargest {

    fn new(k: i32, nums: Vec<i32>) -> Self {
        Self { rank: nums, order: (k - 1) as usize }
    }
    
    fn add(&mut self, val: i32) -> i32 {
        self.rank.push(val);
        self.rank.sort_unstable_by(|n1, n2| n2.cmp(&n1));

        return self.rank[self.order];
    }
}

/**
 * Your KthLargest object will be instantiated and called as such:
 * let obj = KthLargest::new(k, nums);
 * let ret_1: i32 = obj.add(val);
 */
