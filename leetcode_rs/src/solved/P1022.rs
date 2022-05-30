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
 * @lc app=leetcode.cn id=1022 lang=rust
 *
 * [1022] 从根到叶的二进制数之和
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
use std::cell::RefCell;
impl Solution {
    pub fn sum_root_to_leaf(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        return dfs(0, root.unwrap());
    }
}

fn dfs(preval:i32,node:Rc<RefCell<TreeNode>>)->i32{
    let v = node.borrow_mut().val;
    let mut curval = (preval<<1) +v ;
    if node.borrow().left.is_none() && node.borrow().right.is_none(){
        return curval;
    }

    let mut leftv = 0;
    let mut rightv = 0;
    if node.borrow().left.is_some(){
        leftv = dfs(curval, node.borrow().left.clone().unwrap());
    }
    if node.borrow().right.is_some(){
        rightv = dfs(curval, node.borrow().right.clone().unwrap());
    }

    return leftv + rightv;
}
// @lc code=end
#[test]
fn test(){
    let root = Some(Rc::new(RefCell::new(TreeNode{
        val:1,
        left:Some((Rc::new(RefCell::new(TreeNode{
            val:0,
            left:Some(Rc::new(RefCell::new(TreeNode::new(0)))),
            right:Some(Rc::new(RefCell::new(TreeNode::new(1)))),
        })))),
        right:Some(Rc::new(RefCell::new(TreeNode{
            val:1,
            left:Some(Rc::new(RefCell::new(TreeNode::new(0)))),
            right:Some(Rc::new(RefCell::new(TreeNode::new(1)))),
        })))
    })));

    let ans = 22;
    let res = Solution::sum_root_to_leaf(root);
    assert_eq!(res,ans);
}
