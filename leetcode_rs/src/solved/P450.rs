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
 * @lc app=leetcode.cn id=450 lang=rust
 *
 * [450] 删除二叉搜索树中的节点
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
    pub fn delete_node(root: Option<Rc<RefCell<TreeNode>>>, key: i32) -> Option<Rc<RefCell<TreeNode>>> {
        let mut node = root.clone();
        let mut nodeparent = None;
        let mut childflag = None;
        while node.is_some(){
            let nodeval = node.clone().unwrap().borrow().val;
            if nodeval == key{
                break;
            }
            nodeparent = node.clone();
            if nodeval > key{
                node = node.unwrap().borrow().left.clone();
                childflag = Some(-1);
            }
            if nodeval < key{
                node =  node.unwrap().borrow().right.clone();
                childflag = Some(1);
            }
        }

        if node.is_none(){
            return root;
        }

        if let Some(ref v) = node {
            if v.borrow().left.is_none() && v.borrow().right.is_none(){
                if let Some(ref vparent) = nodeparent{
                    if childflag.unwrap()<0{
                        vparent.borrow_mut().left = None;
                    }
                    if childflag.unwrap()>0{
                        vparent.borrow_mut().right = None;
                    }
                }else{
                    return None;
                }
            }else if v.borrow().left.is_none(){
                if let Some(ref vparent) = nodeparent{
                    if childflag.unwrap()<0{
                        vparent.borrow_mut().left = v.borrow().right.clone();
                    }
                    if childflag.unwrap()>0{
                        vparent.borrow_mut().right = v.borrow().right.clone();
                    }
                }else{
                    return v.borrow().right.clone();
                }
            }else if v.borrow().right.is_none(){
                if let Some(ref vparent) = nodeparent{
                    if childflag.unwrap()<0{
                        vparent.borrow_mut().left = v.borrow().left.clone();
                    }
                    if childflag.unwrap()>0{
                        vparent.borrow_mut().right = v.borrow().left.clone();
                    }
                }else{
                    return v.borrow().left.clone();
                }
            }else{
                let mut rightminnode = v.borrow().right.clone();
                let mut rightmindpar = node.clone();
                while rightminnode.is_some(){
                    rightmindpar = rightminnode.clone();
                    rightminnode = rightminnode.unwrap().borrow().left.clone();
                }
                rightmindpar.unwrap().borrow_mut().left = v.borrow().left.clone();

                if let Some(ref vparent) = nodeparent{
                    if childflag.unwrap()<0{
                        vparent.borrow_mut().left = v.borrow().right.clone();
                    }
                    if childflag.unwrap()>0{
                        vparent.borrow_mut().right = v.borrow().right.clone();
                    }
                }else{
                    return v.borrow().right.clone();
                }
            }
        }

        return root;
    }
}
// @lc code=end

