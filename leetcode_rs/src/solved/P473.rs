use super::Solution;
/*
 * @lc app=leetcode.cn id=473 lang=rust
 *
 * [473] 火柴拼正方形
 */

// @lc code=start
impl Solution {
    pub fn makesquare(matchsticks: Vec<i32>) -> bool {
        if matchsticks.len()<4{
            return false;
        }
        let totallen:i32 = matchsticks.iter().sum();

        if totallen %4 != 0{
            return false;
        }
        let linelen = totallen/4;

        let mut dp = vec![-1;1<<matchsticks.len()];

        dp[0] = 0;

        for s in 1..dp.len(){
            for (i,&v) in matchsticks.iter().enumerate(){
                if s >>i &1 == 0{
                    continue;
                }
                let s1 = s^(1<<i);
                if dp[s1]>=0 && dp[s1]+v <= linelen{
                    dp[s] = (dp[s1]+v)%linelen;
                    break;
                }
            }
        }

        return dp[dp.len()-1] == 0;
    }
}
// @lc code=end

