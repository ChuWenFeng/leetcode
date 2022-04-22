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
 * @lc app=leetcode.cn id=662 lang=rust
 *
 * [662] 二叉树最大宽度
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
use std::collections::VecDeque;
impl Solution {
    pub fn width_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_none(){
            return 0;
        }
        let mut curr = vec![];
        let mut ans = 1;
        

        ans as i32
    }
}
// @lc code=end

// #[test]
// fn test(){
//     let mut nums = vec![1,3,2,5,-1,-1,9,6,-1,7];
//     let mut root = Rc::new(RefCell::new(TreeNode::new(nums[0])));
//     generate_tree(&nums, root.clone(), 0);
//     // println!("{:#?}",root);
//     let ans = 4;
//     let res = Solution::width_of_binary_tree(Some(root));
//     assert_eq!(res,ans);
// }

// fn generate_tree(nums:&Vec<i32>,node:Rc<RefCell<TreeNode>>,idx:usize){
//     if idx >= nums.len(){
//         return;
//     }
//     let left = idx*2+1;
//     let right = idx*2+2;
//     if left < nums.len(){
//         if nums[left] != -1{
//             let chilenode = Rc::new(RefCell::new(TreeNode::new(nums[left])));
//             generate_tree(nums, chilenode.clone(), left);
//             node.borrow_mut().left = Some(chilenode);
//         }
//         if nums[right] != -1{
//             let chilenode = Rc::new(RefCell::new(TreeNode::new(nums[right])));
//             generate_tree(nums, chilenode.clone(), right);
//             node.borrow_mut().right = Some(chilenode);
//         }
//     }
// }