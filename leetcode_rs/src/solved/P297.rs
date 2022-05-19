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
 * @lc app=leetcode.cn id=297 lang=rust
 *
 * [297] 二叉树的序列化与反序列化
 */

use std::borrow::Borrow;
use std::fmt::format;
use std::ops::Index;
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
use std::str::Chars;
struct Codec {
}

/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Codec {
    fn new() -> Self {
        Codec { }
    }

    fn serialize(&self, root: Option<Rc<RefCell<TreeNode>>>) -> String {
        if let Some(node) = root{
            let left = format!("({})",self.serialize(node.borrow_mut().left.take()));
            let right = format!("({})",self.serialize(node.borrow_mut().right.take()));
            return format!("{}{}{}",left,node.borrow_mut().val,right);
        }else{
            return "X".to_string();
        }
    }
	
    fn deserialize(&self, data: String) -> Option<Rc<RefCell<TreeNode>>> {  
        // let mut list:Vec<char> = data.chars().collect();
        self.parse(&mut data.chars())
    }

    fn parse(&self,data:&mut Chars)-> Option<Rc<RefCell<TreeNode>>>{
        let mut node = TreeNode::new(0);
        if let Some(c) = data.next(){
            match c{
                'X'=>{
                    return None;
                },
                '('=>{
                    node.left = self.parse(data);
                    data.next();
                    let mut v = 0;
                    let mut flag = 1;
                    while let Some(dc) = data.next() {
                            match dc{
                                '-'=>flag = -1,
                                dv @ '0'..='9' =>{
                                    v = v*10 + dv as i32 - '0' as i32;
                                },
                                _=>break,
                            }
                    }
                    node.val = v*flag;
                    node.right = self.parse(data);
                    data.next();
                }
                _=>{}
            }
        }

        Some(Rc::new(RefCell::new(node)))
        
    }
}

/*
 * Your Codec object will be instantiated and called as such:
 * let obj = Codec::new();
 * let data: String = obj.serialize(strs);
 * let ans: Option<Rc<RefCell<TreeNode>>> = obj.deserialize(data);
 */
// @lc code=end

#[test]
fn test(){
    let root = Some(Rc::new(RefCell::new(TreeNode{
        val:1,
        left:Some(Rc::new(RefCell::new(TreeNode{val:2,left:None,right:None}))),
        right:Some(Rc::new(RefCell::new(TreeNode{
            val:3,
            left:Some(Rc::new(RefCell::new(TreeNode::new(4)))),
            right:Some(Rc::new(RefCell::new(TreeNode::new(5))))
        }))),
    })));
    let mut codec = Codec::new();
    let ser = codec.serialize(root.clone());
    let node = codec.deserialize(ser);
    assert_eq!(node,root);
}