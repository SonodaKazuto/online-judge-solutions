// LeetCode 653. Two Sum IV - Input is a BST
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
        let mut history: Vec<i32> = Vec::new();
        if let Some(node) = root {
            history = Self::traverse(node.borrow().left.clone());
            history.push(node.borrow().val);
            history.append(Self::traverse(node.borrow().right.clone()).as_mut());
        }
        return history;
    }

    pub fn find_target(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> bool {
        if root.is_none() { return false; }

        let values: Vec<i32> = Self::traverse(root);
        for v in values.clone().into_iter() {
            let rest: i32 = k - v;
            if (rest != v) && values.contains(&rest) { return true; }
        }
        return false;
    }
}
