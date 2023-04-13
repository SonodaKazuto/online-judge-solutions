// LeetCode 501. Find Mode in Binary Search Tree
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
use std::collections::HashMap;
impl Solution {
    pub fn traverse(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut elements: Vec<i32> = Vec::new();
        if let Some(node) = root {
            let left_node = node.borrow().left.clone();
            let right_node = node.borrow().right.clone();
            elements.append(&mut Self::traverse(left_node));
            elements.push(node.borrow().val);
            elements.append(&mut Self::traverse(right_node));
        }

        return elements;
    }

    pub fn find_mode(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut elements: Vec<i32> = Self::traverse(root);
        let mut frequency: HashMap<i32, i32> = HashMap::new();
        for e in elements.into_iter() { *frequency.entry(e).or_insert(1) += 1; }

        let max_frequency = frequency.iter().max_by_key(|(_, v)| *v).map(|(_, v)| v).unwrap();
        return frequency.iter().filter_map(|(&k, v)| if v == max_frequency { Some(k) } else { None }).collect::<Vec<i32>>();
    }
}
