
use super::Solution;

/*
 * @lc app=leetcode.cn id=380 lang=rust
 *
 * [380] O(1) 时间插入、删除和获取随机元素
 */

// @lc code=start
use std::collections::HashMap;
struct RandomizedSet {
    nums:Vec<i32>,
    indices:HashMap<i32,usize>,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl RandomizedSet {

    fn new() -> Self {
        RandomizedSet { 
            nums:vec![],
            indices:HashMap::new(),
         }
    }
    
    fn insert(&mut self, val: i32) -> bool {
        let idx = self.nums.len();
        
        match self.indices.get(&val){
            Some(_)=>{
                return false;
            },
            None =>{
                self.indices.insert(val, idx);
                self.nums.push(val);
                return true;
            },
        }
    }
    
    fn remove(&mut self, val: i32) -> bool {

        match self.indices.remove(&val){
            Some(idx)=>{
                let v = *self.nums.last().unwrap();
                self.nums[idx] = v;
                self.indices.insert(v, idx);
                self.nums.pop();
                self.indices.remove(&val);
                return true;
            },
            None =>{
                return false;
            }
        }
        
    }
    
    // fn get_random(&self) -> i32 {
    //     let r = rand::random::<usize>() % self.nums.len();
    //     self.nums[r]
    // }
}

/*
 * Your RandomizedSet object will be instantiated and called as such:
 * let obj = RandomizedSet::new();
 * let ret_1: bool = obj.insert(val);
 * let ret_2: bool = obj.remove(val);
 * let ret_3: i32 = obj.get_random();
 */
// @lc code=end

