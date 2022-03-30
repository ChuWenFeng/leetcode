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
 * @lc app=leetcode.cn id=23 lang=rust
 *
 * [23] 合并K个升序链表
 */

// @lc code=start
// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }

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
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        use std::collections::BinaryHeap;
        use std::cmp::Reverse;
        let mut head:Option<Box<ListNode>> = None;
        let mut ptr = &mut head;

        let mut minheap:BinaryHeap<Reverse<(i32,usize)>> = BinaryHeap::new();
        let mut lists:Vec<&Option<Box<ListNode>>> = lists.iter().collect();
        
        for (idx,head) in lists.iter_mut().enumerate(){
            if let Some(node) = head{
                minheap.push(Reverse((node.val,idx)));
                *head = &node.next;
            }
        }
        let mut count = lists.len();
        while let Some(Reverse((val,idx))) = minheap.pop(){
            let new_node = Box::new(ListNode::new(val));
            if let Some(node) = lists[idx]{
                minheap.push(Reverse((node.val,idx)));
                lists[idx] = &node.next;
            }
            if let Some(tail) = ptr{
                tail.next = Some(new_node);
                ptr = &mut tail.next;
            }else{
                *ptr = Some(new_node);
            }
        }

        return head;
    }
}
// @lc code=end

fn generate_linknode(list:Vec<i32>)->Option<Box<ListNode>>{
    let mut head = None;
    for &i in list.iter().rev(){
        let mut node = ListNode{val:i,next:head.clone()};
        head = Some(Box::new(node));
    }
    return head;
}

#[test]
fn test(){
    let mut list = vec![vec![1,4,5],vec![1,3,4],vec![2,6]];
    let mut input = vec![];
    for i in list{
        input.push(generate_linknode(i));
    }
    let ans = vec![1,1,2,3,4,4,5,6];

    let mut res = Solution::merge_k_lists(input);
    let mut count = 0;

    while res.is_some() {
        assert_eq!(res.clone().unwrap().val,ans[count]);
        res = res.unwrap().next;
        count+=1;
    }
}

#[test]
fn test_linknode(){
    let mut list = vec![vec![1,4,5],vec![1,3,4],vec![2,6]];
    let mut input = vec![];
    for i in list{
        input.push(generate_linknode(i));
    }
    let mut res = Solution::merge_k_lists(input);

    while res.is_some(){
        println!("{}",res.clone().unwrap().val);
        res = res.unwrap().next;
    }

}