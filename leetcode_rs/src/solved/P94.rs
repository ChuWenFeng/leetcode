use super::Solution;
/*
 * @lc app=leetcode.cn id=94 lang=rust
 *
 * [94] 二叉树的中序遍历
 */

// @lc code=start
// Definition for a binary tree node.
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
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut ans = vec![];

        fn dp(node:&Option<Rc<RefCell<TreeNode>>>,ans:&mut Vec<i32>){
            if let Some(node) = node{
                dp(&node.borrow().left,ans);
                ans.push(node.borrow().val);
                dp(&node.borrow().right,ans);
            }
        }

        dp(&root,&mut ans);

        return ans;
    }
}
// @lc code=end

