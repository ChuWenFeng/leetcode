use super::Solution;
/*
 * @lc app=leetcode.cn id=139 lang=rust
 *
 * [139] 单词拆分
 */

// @lc code=start
use std::collections::{ HashSet};
impl Solution {
    pub fn word_break_1(s: String, word_dict: Vec<String>) -> bool {
        let len = s.len();
        let mut table = HashSet::new();
        let mut dp = vec![false;len+1];
        dp[0] = true;
        for i in word_dict{
            table.insert(i);
        }
        for i in 1..=len{
            for j in 0..i{
                if dp[j] && table.get(&s[j..i].to_string()).is_some(){
                    dp[i] = true;
                    break;
                }
            }
        }

        dp[len]
    }
}
// @lc code=end

