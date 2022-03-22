use super::Solution;
/*
 * @lc app=leetcode.cn id=122 lang=rust
 *
 * [122] 买卖股票的最佳时机 II
 */

// @lc code=start
impl Solution {
    pub fn max_profit_2(prices: Vec<i32>) -> i32 {
        let len = prices.len();
        if len <1{
            return 0;
        }
        let mut hold_stock = -prices[0];
        let mut sale_stock = 0;

        for &i in prices.iter().skip(1){
            let pre_hold = hold_stock;
            let pre_sale = sale_stock;
            if pre_sale > pre_hold + i{//不持有股票
                sale_stock = pre_sale;
            }else{
                sale_stock = pre_hold + i;
            }

            if pre_hold > pre_sale - i{//持有股票
                hold_stock = pre_hold;
            }else{
                hold_stock = pre_sale - i;
            }

        }
        

        return sale_stock;
    }
}
// @lc code=end

