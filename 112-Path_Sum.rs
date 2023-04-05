// LeetCode 112. Path Sum
// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
// 
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
        if let Some(node) = root {
            let mut left = node.borrow().left.clone();
            let mut right = node.borrow().right.clone();
            let difference: i32 = target_sum - node.borrow().val;
            if (difference == 0) & left.is_none() & right.is_none() { return true; }
            return Solution::has_path_sum(left, difference) | 
                   Solution::has_path_sum(right, difference);
        }
        return false;
    }
}
