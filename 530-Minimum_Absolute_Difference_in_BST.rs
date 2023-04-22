// LeetCode 530. Minimum Absolute Difference in BST
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
    pub fn traverse(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut values: Vec<i32> = Vec::new();
        if let Some(node) = root {
            values.push(node.borrow().val);
            values.append(Self::traverse(node.borrow().left.clone()).as_mut());
            values.append(Self::traverse(node.borrow().right.clone()).as_mut());
        }
        return values;
    } 
    pub fn get_minimum_difference(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut values: Vec<i32> = Self::traverse(root);
        
        values.sort_unstable();
        return values.windows(2).map(|w| (w[0] - w[1]).abs()).min().unwrap_or(0);
    }
}
