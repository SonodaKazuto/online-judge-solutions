// LeetCode 203. Remove Linked List Elements
// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
// 
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
impl Solution {
    pub fn remove_elements(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
        if head.is_none() { return head; }

        let mut new_head = head;
        let mut current_node = &mut new_head;

        loop {
            match current_node {
                None => break,
                Some(node) if node.val == val => *current_node = node.next.take(),
                Some(node) => current_node = &mut node.next,
            }
        }

        return new_head;
    }
}
