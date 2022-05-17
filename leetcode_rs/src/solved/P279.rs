use super::Solution;
/*
 * @lc app=leetcode.cn id=279 lang=rust
 *
 * [279] 完全平方数
 */

// @lc code=start
impl Solution {
    pub fn num_squares(n: i32) -> i32 {
        let n = n as usize;
        let mut dp = vec![0;n as usize + 1];
        for i in 1..=n{
            let mut minn = usize::MAX;
            let mut j = 1;
            while  j*j <= i{
                minn = minn.min(dp[i-j*j]);
                j+=1;
            }
            dp[i] = minn+1;
        }
        return dp[n] as i32;
    }
}
// @lc code=end

