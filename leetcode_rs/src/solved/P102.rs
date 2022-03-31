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
 * @lc app=leetcode.cn id=102 lang=rust
 *
 * [102] 二叉树的层序遍历
 */

// @lc code=start
// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }

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
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut ans = vec![];
        let mut queue = std::collections::VecDeque::new();
        let mut layercount = 0;
        if let Some(node) = root{
            let val = node.borrow().val;
            ans.push(vec![val]);
            if let Some(left) = &node.borrow().left{
                queue.push_back(left.clone());
                layercount+=1;
            }
            if let Some(right) = &node.borrow().right{
                queue.push_back(right.clone());
                layercount+=1;
            }
        }else{
            return ans;
        }
        let mut count = layercount;
        layercount = 0;

        while queue.len()>0{
            let mut layernode = vec![];
            while count >0{
                if let Some(node) = queue.pop_front(){
                    layernode.push(node.borrow().val);
                    if let Some(left) = &node.borrow().left{
                        queue.push_back(left.clone());
                        layercount+=1;
                    }
                    if let Some(right) = &node.borrow().right{
                        queue.push_back(right.clone());
                        layercount+=1;
                    }
                }
                count-=1;
            }
            ans.push(layernode);
            count = layercount;
            layercount=0;
        }

        return ans;
    }
}
// @lc code=end

