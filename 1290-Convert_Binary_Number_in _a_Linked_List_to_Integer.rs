// LeetCode 1290. Convert Binary Number in a Linked List to Integer
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
    pub fn get_decimal_value(head: Option<Box<ListNode>>) -> i32 {
        let mut decimal_value: i32 = 0;
        let mut cur = head.clone();
        
        while let Some(node) = cur {
            decimal_value *= 2;
            decimal_value += node.val;
            cur = node.next;
        }

        return decimal_value;
    }
}
