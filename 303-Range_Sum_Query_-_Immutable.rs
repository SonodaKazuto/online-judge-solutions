// LeetCode 303. Range Sum Query - Immutable
struct NumArray {
    array: Vec<i32>
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NumArray {

    fn new(nums: Vec<i32>) -> Self {
        return Self { array: nums }
    }
    
    fn sum_range(&self, left: i32, right: i32) -> i32 {
        let mut sum_range: i32 = 0;
        if (left <= right) && !self.array.is_empty() {
            for i in left..=right { sum_range += self.array[i as usize]; }
        }
        return sum_range;
    }
}

/**
 * Your NumArray object will be instantiated and called as such:
 * let obj = NumArray::new(nums);
 * let ret_1: i32 = obj.sum_range(left, right);
 */
