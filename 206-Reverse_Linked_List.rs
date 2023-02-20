// LeetCode 206. Reverse Linked List
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
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let (mut rev_head, mut cur) = (None, head);
        while let Some(mut tmp) = cur {
            cur = tmp.next;
            tmp.next = rev_head;
            rev_head = Some(tmp);
        }
        return rev_head
    }
}
