use super::Solution;
/*
 * @lc app=leetcode.cn id=121 lang=rust
 *
 * [121] 买卖股票的最佳时机
 */

// @lc code=start
impl Solution {
    pub fn max_profit_1(prices: Vec<i32>) -> i32 {
        let mut ans = 0;
        let mut min = i32::MAX;
        for &i in prices.iter(){
            if i < min{
                min = i;
                continue;
            }
            if i - min > ans{
                ans = i - min;
            }
        }

        return ans;
    }
}
// @lc code=end

