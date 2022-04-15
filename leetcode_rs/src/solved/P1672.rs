use super::Solution;
/*
 * @lc app=leetcode.cn id=1672 lang=rust
 *
 * [1672] 最富有客户的资产总量
 */

// @lc code=start
impl Solution {
    pub fn maximum_wealth(accounts: Vec<Vec<i32>>) -> i32 {

        accounts.iter().fold(0, |max,x|{
            x.iter().fold(0, |sum,x|{
                sum+x
            }).max(max)
        })
    }
}
// @lc code=end

