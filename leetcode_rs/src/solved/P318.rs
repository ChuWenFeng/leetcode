use super::Solution;
/*
 * @lc app=leetcode.cn id=318 lang=rust
 *
 * [318] 最大单词长度乘积
 */

// @lc code=start
impl Solution {
    pub fn max_product(words: Vec<String>) -> i32 {
        let mut list = vec![];
        for word in words{
            let mut bit = 0;
            for c in word.chars(){
                bit |= 1<<(c as i32 - 'a' as i32);
            }
            list.push((bit,word.len()));
        }

        let mut ans = 0;
        for i in 0..list.len(){
            for j in i+1..list.len(){
                if list[i].0 & list[j].0 ==0{
                    ans = ans.max(list[i].1*list[j].1);
                }
            }
        }

        ans as i32
    }
}
// @lc code=end

