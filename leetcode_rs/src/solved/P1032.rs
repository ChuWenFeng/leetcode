use std::sync::Arc;

use super::Solution;


/*
 * @lc app=leetcode.cn id=1032 lang=rust
 *
 * [1032] 字符流
 */

// @lc code=start
struct StreamChecker {
    // words:Vec<String>,
    root:Node,
    cstack:Vec<usize>,
}
#[derive(Clone)]
struct Node{
    isworld:bool,
    children:Vec<Option<Box<Node>>>,
}
impl Node{
    fn new()->Self{
        Node { isworld: false, children: vec![None;26] }
    }
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl StreamChecker {

    fn new(words: Vec<String>) -> Self {
        let mut root = Node::new();
        for str in words{
            let mut curr:&mut Node = &mut root;

            for c in str.chars().rev(){
                let idx = c as usize - 'a' as usize;
                if curr.children[idx].is_none(){
                    curr.children[idx] = Some(Box::new(Node::new()));
                }
                curr = curr.children[idx].as_mut().unwrap();
            }
            curr.isworld = true;
        }
        StreamChecker { root ,cstack:vec![]}
    }
    
    fn query(&mut self, letter: char) -> bool {

        let idx = letter as usize - 'a' as usize;
        self.cstack.push(idx);

        let mut curr = &self.root;
        for &c in self.cstack.iter().rev(){
            if curr.children[c].is_none(){
                return false;
            }
            curr = curr.children[c].as_ref().unwrap();
            if curr.isworld{
                return true;
            }
        }
        

        return false;
    }
}

/*
 * Your StreamChecker object will be instantiated and called as such:
 * let obj = StreamChecker::new(words);
 * let ret_1: bool = obj.query(letter);
 */
// @lc code=end

