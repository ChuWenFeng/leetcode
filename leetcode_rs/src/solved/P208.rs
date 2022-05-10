
use super::Solution;
/*
 * @lc app=leetcode.cn id=208 lang=rust
 *
 * [208] 实现 Trie (前缀树)
 */

// @lc code=start
#[derive(Default)]
struct Trie {
    child:[Option<Box<Trie>>;26],
    isword:bool,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Trie {

    fn new() -> Self {
        Self::default()
    }
    
    fn insert(&mut self, word: String) {
        let mut curr = self;
        for c in word.chars(){
            let idx = c as usize - 'a' as usize;
            curr = curr.child[idx].get_or_insert(Box::new(Self::default()));
        }
        curr.isword = true;
    }
    
    fn search(&self, word: String) -> bool {
        let mut curr = self;
        for c in word.chars(){
            let idx = c as usize - 'a' as usize;
            match &curr.child[idx]{
                Some(n)=>curr = n,
                None=>return false,
            }
        }
        
        curr.isword
    }
    
    fn starts_with(&self, prefix: String) -> bool {
        let mut curr = self;
        for c in prefix.chars(){
            let idx = c as usize - 'a' as usize;
            match &curr.child[idx]{
                Some(n)=>curr = n,
                None=>return false,
            }
        }
        
        true
    }
}

/*
 * Your Trie object will be instantiated and called as such:
 * let obj = Trie::new();
 * obj.insert(word);
 * let ret_2: bool = obj.search(word);
 * let ret_3: bool = obj.starts_with(prefix);
 */
// @lc code=end

