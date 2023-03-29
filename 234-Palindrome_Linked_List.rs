// LeetCode 234. Palindrome Linked List
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
    pub fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
        if head.is_none() { return false; }

        let mut cur = head;
        let mut list: Vec<i32> = Vec::new();
        while let Some(node) = cur {
            list.push(node.val);
            cur = node.next;
        }
        if list.len() == 1 { return true; }
        let mut reverse_list: Vec<i32> = list.clone();
        reverse_list.reverse();
        return list == reverse_list;
    }
}
