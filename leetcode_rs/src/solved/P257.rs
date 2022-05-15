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
 * @lc app=leetcode.cn id=257 lang=rust
 *
 * [257] 二叉树的所有路径
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
    pub fn binary_tree_paths(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<String> {

        let mut stack:Vec<Option<Rc<RefCell<TreeNode>>>> = vec![];
        let mut vallist:Vec<i32> = vec![];
        let mut ans = vec![];
        dfs(root.unwrap(),&mut vallist,&mut ans);
        ans
    }
}

fn dfs(node:Rc<RefCell<TreeNode>>,vallist:&mut Vec<i32>,ans:&mut Vec<String>){
    vallist.push(node.borrow().val);
    if node.borrow().left.is_none() && node.borrow().right.is_none(){
        ans.push(get_path(vallist));
        return;
    }
    if node.borrow().left.is_some(){
        dfs(node.borrow().left.clone().unwrap(), vallist, ans);
        vallist.pop();
    }
    if node.borrow().right.is_some(){
        dfs(node.borrow().right.clone().unwrap(), vallist, ans);
        vallist.pop();
    }
}

fn get_path(v:&Vec<i32>)->String{
    let mut path = String::new();

    for i in 0..v.len()-1{
        path.extend(v[i].to_string().chars());
        path.push_str("->");
    }
    path.extend(v[v.len()-1].to_string().chars());

    path
}
// @lc code=end

