// LeetCode 606. Construct String from Binary Tree
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
    pub fn traverse(root: Option<Rc<RefCell<TreeNode>>>) -> String {
        let mut history: String = String::new();
        if let Some(node) = root {
            history += &node.borrow().val.to_string();
            
            let (left_tree, right_tree) = (node.borrow().left.clone(), node.borrow().right.clone());
            if left_tree.is_some() { 
                history += &format!("({})", Self::traverse(left_tree));
                if right_tree.is_some() { history += &format!("({})", Self::traverse(right_tree)); }
            }
            else {
                if right_tree.is_some() { history += &format!("()({})", Self::traverse(right_tree)); }
            }
        }
        return history;
    }
    
    pub fn tree2str(root: Option<Rc<RefCell<TreeNode>>>) -> String {
        return Self::traverse(root);
    }
}
