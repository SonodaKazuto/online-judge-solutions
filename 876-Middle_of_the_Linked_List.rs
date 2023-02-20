// LeetCode 876. Middle of the Linked List
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
    pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let (mut cur, mut count) = (head.clone(), 0);
        while let Some(node) = cur { 
            cur = node.next;
            count +=1;
        }

        let target_pos: i32 = count / 2;
        count = 0;
        cur = head.clone();
        while let Some(node) = cur {
            if count == target_pos { return Some(node); }
            cur = node.next;
            count +=1;
        }
        return None;
    }
}
