use std::vec;

use super::Solution;
/*
 * @lc app=leetcode.cn id=887 lang=rust
 *
 * [887] 鸡蛋掉落
 */

// @lc code=start
impl Solution {
    pub fn super_egg_drop(k: i32, n: i32) -> i32 {
        // Self::super_egg_drop_math(k,n)
        Self::super_egg_drop_traver(k,n)
    }
    pub fn super_egg_drop_math(k: i32, n: i32) -> i32 {
        let k = k as usize; 
        let n = n as usize; 
        let mut ans = 1;
        if n == 1{return ans;}
        let mut dp = vec![vec![0;k+1];n+1];
        for i in 1..=k{
            dp[1][i] = 1;
        }
        for i in 2..=n{
            for j in 1..=k{
                dp[i][j] = 1 + dp[i-1][j-1] + dp[i-1][j];
            }
            if dp[i][k] >= n{
                ans = i as i32;
                break;
            }
        }   
        return ans;
    }
    pub fn super_egg_drop_traver(k: i32, n: i32) -> i32 {
        let mut ans = 1;
        if n == 1 {return 1};
        let k = k as usize;
        let n = n as usize;
        let mut dp = vec![vec![0;n+1];k+1];
        for i in 1..=n{
            dp[1][i] = i;
        }
        for i in 2..=k{
            for j in 1..=n{
                //初始化最坏情况，dp[i][j-1]表示未摔坏，dp[i-1]][j-1]表示摔坏，根据j-1层的值+1取最差值
                // dp[i][j] = 1 + if dp[i][j-1] > dp[i-1][j-1] {dp[i][j-1]}else{dp[i-1][j-1]};
                dp[i][j] = usize::MAX-1;
                for k in 2..=j{
                    //k层摔坏坏则取dp[i-1][k-1],没坏则取dp[i][j-k]
                    let m = if dp[i-1][k-1] > dp[i][j-k]{dp[i-1][k-1]}else{dp[i][j-k]};
                    dp[i][j] = if dp[i][j] < 1 + m {dp[i][j]} else {1+m};
                }
            }
        }
        ans = dp[k][n] as i32; 
        return ans;
    }
}
// @lc code=end

#[test]
fn test(){
    let (k,n) = (5,5000);
    let a = Solution::super_egg_drop(k,n);
    let ans = Solution::super_egg_drop_math(k, n);
    assert_eq!(a,ans)
}