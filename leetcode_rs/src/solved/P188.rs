use super::Solution;
/*
 * @lc app=leetcode.cn id=188 lang=rust
 *
 * [188] 买卖股票的最佳时机 IV
 */

// @lc code=start
impl Solution {
    pub fn max_profit(k: i32, prices: Vec<i32>) -> i32 {
        let mut ans = 0;
        
        let mut k  = k as usize;
        let len = prices.len();
        if k > len/2{
            k = len/2;
        }
        if len < 1{
            return 0;
        }
        let mut buy = vec![vec![0;k+1];len+1];
        let mut sell = vec![vec![0;k+1];len+1];
        buy[0][0] = -prices[0];
        for i in 1..len{
            buy[0][i] = i32::MIN;
            sell[0][i] = i32::MIN;
        }
        let max = |a,b|->i32{
            if a > b{
                return a;
            }else{
                return b;
            }
        };

        for (idx,&val) in prices.iter().enumerate().skip(1){
            for i in 1..=k{
                buy[idx][i] = max(buy[idx-1][i],sell[idx-1][i]-val);
                sell[idx][i] = max(sell[idx-1][i],buy[idx-1][i-1]+val);
            }
        }

        return sell[len][k];
    }
}
// @lc code=end
#[test]
fn test(){
    let input = vec![2,4,1];
    let ans = 2;
    let res = Solution::max_profit(2, input);
    assert_eq!(res,ans);
}