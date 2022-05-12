use super::Solution;
/*
 * @lc app=leetcode.cn id=944 lang=rust
 *
 * [944] 删列造序
 */

// @lc code=start
impl Solution {
    pub fn min_deletion_size(strs: Vec<String>) -> i32 {
        let len = strs[0].len();
        let mut list = vec![vec![];len];

        for str in strs.iter(){
            for (i,c) in str.chars().enumerate(){
                list[i].push(c as u8);
            }
        }
        let mut ans = 0;
        for j in 0..len{
            for i in 1..list[0].len(){
                if list[j][i-1] > list[j][i]{
                    ans+=1;
                    break;
                }
            }
        }


        ans
    }
}
// @lc code=end

