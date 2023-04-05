// LeetCode 111. Minimum Depth of Binary Tree
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
use std::cmp;
impl Solution {
    pub fn min_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(node) = root {
            let left_depth: i32 = Self::min_depth(node.borrow().left.clone());
            let right_depth: i32 = Self::min_depth(node.borrow().right.clone());
            if (left_depth == 0) | (right_depth == 0) { return 1 + cmp::max(left_depth, right_depth); }
            return 1 + cmp::min(left_depth, right_depth);
        }
        return 0;
    }
}
