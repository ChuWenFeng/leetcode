use super::Solution;
/*
 * @lc app=leetcode.cn id=312 lang=rust
 *
 * [312] 戳气球
 */

// @lc code=start
impl Solution {
    pub fn max_coins(nums: Vec<i32>) -> i32 {
        let len = nums.len();
        let mut dp = vec![vec![0;len+2];len+2];
        let mut val = vec![0;len+2];
        for (i,&v) in nums.iter().enumerate(){
            val[i+1] = v;
        }
        val[0] = 1;
        val[len+1] = 1;

        for i in (0..len).rev(){
            for j in i+2..len+2{
                for k in i+1..j{
                    let mut sum = val[i]*val[k]*val[j];
                    sum += dp[i][k] + dp[k][j];
                    dp[i][j] = sum.max(dp[i][j]);
                }
            }
        }

        return dp[0][len+1];
    }
}
// @lc code=end

