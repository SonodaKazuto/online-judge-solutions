// LeetCode 144. Binary Tree Preorder Traversal
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
    pub fn preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut history: Vec<i32> = Vec::new();

        if let Some(node) = root {
            history.push(node.borrow().val);
            history.append(Solution::preorder_traversal(node.borrow().left.clone()).as_mut());
            history.append(Solution::preorder_traversal(node.borrow().right.clone()).as_mut());
        }

        return history;
    }
}
