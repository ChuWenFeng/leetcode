use super::Solution;
/*
 * @lc app=leetcode.cn id=6008 lang=rust
 *
 * [6008] 统计包含给定前缀的字符串
 */

// @lc code=start
impl Solution {
    pub fn prefix_count(words: Vec<String>, pref: String) -> i32 {
        let mut count = 0;
        for word in words{
            if word.starts_with(&pref){
                count+=1;
            }
        }
        return count;
    }
}
// @lc code=end

