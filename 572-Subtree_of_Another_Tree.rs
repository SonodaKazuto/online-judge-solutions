// LeetCode 572. Subtree of Another Tree
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
    pub fn is_subtree(root: Option<Rc<RefCell<TreeNode>>>, sub_root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if root == sub_root { return true; }
        if let Some(node) = root {
            let left_tree = node.borrow().left.clone();
            let right_tree = node.borrow().right.clone();
            return Self::is_subtree(left_tree, sub_root.clone()) || Self::is_subtree(right_tree, sub_root.clone());
        }
        return false;
    }
}
