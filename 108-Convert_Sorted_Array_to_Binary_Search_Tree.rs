// LeetCode 108. Convert Sorted Array to Binary Search Tree
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
    pub fn build_tree(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        if nums.is_empty() { return None; }

        let (left, rest) = nums.split_at(nums.len() / 2);
        let (root_val, right) = rest.split_first().unwrap();
        return Some(Rc::new(RefCell::new(TreeNode {
            val: *root_val,
            left: Self::build_tree(left.to_vec()),
            right: Self::build_tree(right.to_vec())
        })));
    }

    pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        if nums.is_empty() { return None; }
        return Self::build_tree(nums);
    }
}
