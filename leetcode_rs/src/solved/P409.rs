use std::collections::HashMap;

use super::Solution;

/*
 * @lc app=leetcode.cn id=409 lang=rust
 *
 * [409] 最长回文串
 */

// @lc code=start
impl Solution {
    pub fn longest_palindrome(s: String) -> i32 {
        let mut table = vec![0;128];
        for c in s.chars(){
            let idx = c as usize;
            table[idx]+=1;
        }
        let mut ans = 0;
        let mut flag = false;
        for v in table{
            if v&1 == 1{
                flag = true;
            }
            ans+=v/2*2;
        }
        if flag{
            ans+=1;
        }
        ans
    }
}
// @lc code=end

