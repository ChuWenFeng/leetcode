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
 * @lc app=leetcode.cn id=1305 lang=rust
 *
 * [1305] 两棵二叉搜索树中的所有元素
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
    pub fn get_all_elements(root1: Option<Rc<RefCell<TreeNode>>>, root2: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut stack1 = vec![];
        let mut stack2 = vec![];
        let mut root1 = root1;
        let mut root2 = root2;
        let mut ans = vec![];
        while (root1.is_some() || !stack1.is_empty()) && (root2.is_some() || !stack2.is_empty()){
            while root1.is_some(){
                stack1.push(root1.clone().unwrap().clone());
                root1 = root1.unwrap().borrow().left.clone();
            }
            while root2.is_some(){
                stack2.push(root2.clone().unwrap().clone());
                root2 = root2.unwrap().borrow().left.clone();
            }

            if stack1.last().unwrap().borrow().val > stack2.last().unwrap().borrow().val{
                root2 = stack2.pop();
                ans.push(root2.clone().unwrap().borrow().val);
                root2 = root2.unwrap().borrow().right.clone();
            }else{
                root1 = stack1.pop();
                ans.push(root1.clone().unwrap().borrow().val);
                root1 = root1.unwrap().borrow().right.clone();
            }
        }
        addItem(root1, &mut stack1, &mut ans);
        addItem(root2, &mut stack2, &mut ans);
        
        ans
    }
}

fn addItem(mut root:Option<Rc<RefCell<TreeNode>>>,stack:&mut Vec<Rc<RefCell<TreeNode>>>,ans:&mut Vec<i32>){
    while root.is_some() || !stack.is_empty(){
        while root.is_some(){
            stack.push(root.clone().unwrap());
            root = root.unwrap().borrow().left.clone();
        }
        root = stack.pop();
        ans.push(root.clone().unwrap().borrow().val);
        root = root.unwrap().borrow().right.clone();
    }
}
// @lc code=end

