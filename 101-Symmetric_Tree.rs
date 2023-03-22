// LeetCode 101. Symmetric Tree
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
    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(ref node) = root {
            let left_tree = node.borrow().left.clone();
            let right_tree = node.borrow().right.clone();

            node.borrow_mut().left = Solution::invert_tree(right_tree);
            node.borrow_mut().right = Solution::invert_tree(left_tree);
        }
        return root;
    }

    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if let Some(ref node) = root {
            return node.borrow().left == Solution::invert_tree(node.borrow().right.clone());
        }
        return false;
    }
}
