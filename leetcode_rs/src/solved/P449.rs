use super::P140::dfs;
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
 * @lc app=leetcode.cn id=449 lang=rust
 *
 * [449] 序列化和反序列化二叉搜索树
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
struct Codec {
	list:Vec<i32>
}

/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Codec {
    fn new() -> Self {
        Codec { list:vec![] }
    }

    fn serialize(&mut self, root: Option<Rc<RefCell<TreeNode>>>) -> String {
        if root.is_none(){
            return "".to_string();
        }
        self.list.clear();
        self.dfs(root.unwrap());
        self.to_string()
    }

    fn to_string(&self)->String{
        let mut res = "".to_string();
        self.list.iter().fold(&mut res, |res,val|{
            res.push_str(&val.to_string());
            res.push(' ');
            res
        });
        res.pop();
        res
    }

    fn dfs(&mut self,node:Rc<RefCell<TreeNode>>){
        
        if let Some(left) = &node.borrow().left{
            self.dfs(left.clone());
        }
        if let Some(right) = &node.borrow().right{
            self.dfs(right.clone());
        }
        self.list.push(node.borrow().val);

    }
	
    fn deserialize(&mut self, data: String) -> Option<Rc<RefCell<TreeNode>>> {
        if data.is_empty(){
            return None;
        }
        self.list = data.split(' ').into_iter().map(|x|x.parse().unwrap()).collect();
        return self.construct(i32::MIN,i32::MAX)
    }

    fn construct(&mut self,lower:i32,upper:i32)->Option<Rc<RefCell<TreeNode>>>{
        if self.list.len() == 0{
            return None;
        }
        let &val = self.list.last().unwrap();
        if val < lower || val > upper{
            return None;
        }
        self.list.pop();
        return Some(Rc::new(RefCell::new(TreeNode{
            val,
            right:self.construct(val, upper),
            left:self.construct(lower, val),
        })));
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
    let root = Rc::new(RefCell::new(TreeNode{
        val:2,
        left:Some(Rc::new(RefCell::new(TreeNode{
            val:1,
            left:None,
            right:None,
        }))),
        right:Some(Rc::new(RefCell::new(TreeNode{
            val:3,
            left:None,
            right:None,
        }))),
    }));
    let ansstring = "1 3 2".to_string();
    let mut codec = Codec::new();
    let resstring = codec.serialize(Some(root.clone()));
    assert_eq!(resstring,ansstring);
    let resroot = codec.deserialize(resstring);
    assert_eq!(resroot,Some(root));
}