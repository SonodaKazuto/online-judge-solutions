// LeetCode 110. Balanced Binary Tree
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
    pub fn check_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(node) = root {
            let left_depth: i32 = Self::check_balanced(node.borrow().left.clone());
            let right_depth: i32 = Self::check_balanced(node.borrow().right.clone());
            if (left_depth == -1) || (right_depth == -1) || ((left_depth - right_depth).abs() > 1 ) { return -1; }
            return 1 + cmp::max(left_depth, right_depth);
        }
        return 0;
    }

    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        return Self::check_balanced(root) != -1;
    }
}
