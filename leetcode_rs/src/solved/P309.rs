use super::Solution;
/*
 * @lc app=leetcode.cn id=309 lang=rust
 *
 * [309] 最佳买卖股票时机含冷冻期
 */

// @lc code=start
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        if prices.is_empty(){
            return 0;
        }
        let mut pre = (0,0,0);
        let mut cur = (-prices[0],0,0);
        for (i,&v) in prices.iter().enumerate().skip(1){
            pre = cur;
            cur = (0,0,0);
            cur.0 = pre.0.max(pre.2 - prices[i]);
            cur.1 = pre.0 + prices[i];
            cur.2 = pre.1.max(pre.2);
            
        }

        return cur.1.max(cur.2);
    }
}
// @lc code=end
