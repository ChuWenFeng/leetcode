use super::Solution;
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}
/*
 * @lc app=leetcode.cn id=2095 lang=rust
 *
 * [2095] 删除链表的中间节点
 */

// @lc code=start
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
    pub fn delete_middle(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut fast = head.as_ref();
        let mut newhead = Some(Box::new(ListNode::new(head.as_ref().unwrap().val)));
        let mut newidx = &mut newhead;
        let mut idx = head.as_ref().unwrap().next.as_ref();

        let mut pre:&mut Option<Box<ListNode>>;
        while fast.is_some() && fast.unwrap().next.is_some(){
            fast = fast.unwrap().next.as_ref().unwrap().next.as_ref();

            // unsafe{pre = newidx as mut ref Option<BoxM<ListNode>;}

            newidx.as_mut().unwrap().next = Some(Box::new(ListNode::new(idx.unwrap().val)));
            newidx = &mut newidx.as_mut().unwrap().next;
            idx = idx.unwrap().next.as_ref();

        }
        // unsafe{
        //     pre.as_mut().unwrap().next = Some(idx.unwrap().clone());
        // }


        return newhead;
    }
}
// @lc code=end