use super::Solution;
/*
 * @lc app=leetcode.cn id=264 lang=rust
 *
 * [264] 丑数 II
 */

// @lc code=start
impl Solution {
    pub fn nth_ugly_number(n: i32) -> i32 {
        let mut dp = vec![0;n as usize  + 1];
        dp[1] = 1;
        let (mut p1,mut  p2,mut p3) = (1,1,1);
        for i in 2..=n as usize{
            let (x1,x2,x3) = (dp[p1]*2,dp[p2]*3,dp[p3]*5);
            dp[i] = x1.min(x2).min(x3);
            if dp[i] == x1{
                p1 +=1;
            }
            if dp[i] == x2{
                p2 +=1;
            }
            if dp[i] == x3{
                p3 +=1;
            }
        } 

        return dp[n as usize];
    }
}
// @lc code=end
#[test]
fn test(){
    let n = 10;
    let ans = 12;
    let res = Solution::nth_ugly_number(n);
    assert_eq!(res,ans);
}

