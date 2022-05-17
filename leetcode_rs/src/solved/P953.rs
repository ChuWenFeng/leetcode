use super::Solution;
/*
 * @lc app=leetcode.cn id=953 lang=rust
 *
 * [953] 验证外星语词典
 */

// @lc code=start
use std::{collections::HashMap, cmp::Ordering};
impl Solution {
    pub fn is_alien_sorted(words: Vec<String>, order: String) -> bool {
        let mut table = HashMap::new();
        for (i,c) in order.chars().enumerate(){
            table.insert(c, i);
        }

        for i in 0..words.len()-1{
            let mut len = words[i+1].len();
            for (c1,c2) in words[i].chars().zip(words[i+1].chars()){
                let ord1 = table.get(&c1).unwrap();
                let ord2 = table.get(&c2).unwrap();
                match ord1.cmp(ord2) {
                    Ordering::Less=>break,
                    Ordering::Equal=>{},
                    Ordering::Greater=>return false,
                }
                len-=1;
                if words[i].len()>words[i+1].len() && len == 0{
                    return false;
                }
            }
        }

        true
    }
}
// @lc code=end

