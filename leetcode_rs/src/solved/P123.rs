use super::Solution;
/*
 * @lc app=leetcode.cn id=123 lang=rust
 *
 * [123] 买卖股票的最佳时机 III
 */

// @lc code=start
impl Solution {
    pub fn max_profit_3(prices: Vec<i32>) -> i32 {
        let len = prices.len();
        if len < 1{
            return 0;
        }
        let max = |a,b|->i32{
            if a > b{
                return a;
            }else{
                return b;
            }
        };

        let mut b1 = -prices[0];
        let mut s1 = 0;
        let mut b2 = -prices[0];
        let mut s2 = 0;

        for &i in prices.iter().skip(1){
            b1 = max(b1,-i);
            s1 = max(s1,b1+i);
            b2 = max(b2,s1-i);
            s2 = max(s2,b2+i);
        }



        return s2;
    }
}
// @lc code=end

#[test]
fn test(){
    let prices = vec![3,3,5,0,0,3,1,4];
    let ans = 6;
    let res = Solution::max_profit_3(prices);
    assert_eq!(res,ans);
}