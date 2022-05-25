use super::Solution;
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
  pub val: i32,
  pub left: Option<Rc<RefCell<TreeNode>>>,
  pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
  #[inline]
  pub fn new(val: i32) -> Self {
    TreeNode {
      val,
      left: None,
      right: None
    }
  }
}
/*
 * @lc app=leetcode.cn id=965 lang=rust
 *
 * [965] 单值二叉树
 */

// @lc code=start
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
use std::collections::VecDeque;
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn is_unival_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let mut queue = VecDeque::new();
        let v = root.clone().unwrap().borrow_mut().val;
        queue.push_back(root.clone());
        
        while !queue.is_empty() {
            let front = queue.pop_front().unwrap();
            if let Some(node) = front{
                if v != node.borrow_mut().val{
                    return false;
                }
                queue.push_back(node.borrow().left.clone());
                queue.push_back(node.borrow_mut().right.clone());
            }
        }


        return true;
    }
}
// @lc code=end

