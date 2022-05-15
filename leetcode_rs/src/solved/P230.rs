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
 * @lc app=leetcode.cn id=230 lang=rust
 *
 * [230] 二叉搜索树中第K小的元素
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
use std::rc::Rc;
use std::cell::{RefCell,Cell};
impl Solution {
    pub fn kth_smallest(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
        let mut stack = vec![];
        let mut k = k;
        let mut root = root;
        loop {
            while root.is_some(){
                stack.push(root.clone());
                root = root.unwrap().borrow().left.clone();
            }
            root = stack.pop().unwrap();
            k-=1;
            if k == 0{
                return root.unwrap().borrow().val;
            }
            root = root.unwrap().borrow().right.clone();
        }
    }
}
// @lc code=end

