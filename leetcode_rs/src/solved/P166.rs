

use super::Solution;
/*
 * @lc app=leetcode.cn id=166 lang=rust
 *
 * [166] 分数到小数
 */

// @lc code=start
use std::{collections::{HashSet, HashMap}, num};
impl Solution {
    pub fn fraction_to_decimal(numerator: i32, denominator: i32) -> String {
        let mut denominator = denominator as i64;
        let mut numerator = numerator as i64;
        let mut table = HashMap::new();
        let mut ans = String::new();

        if numerator < 0 && denominator > 0{
            numerator = numerator.abs();
            ans.push('-');
        }
        if numerator > 0 && denominator < 0{
            denominator = denominator.abs();
            ans.push('-');
        }

        let inter = numerator / denominator;

        let mut reminder = (numerator % denominator);
        
        let mut decimals = vec![];
        let mut cycle = false;
        let mut idx = 0;
        while reminder != 0 {
            if let Some(&i) = table.get(&reminder){
                cycle = true;
                idx = i;
                break;
            }
            table.insert(reminder,idx);
            reminder *=10;
            let dec = reminder / denominator;
            decimals.push(dec);
            reminder = reminder - (dec * denominator);
            idx+=1;
        }
        ans.push_str(&inter.to_string());
        if !decimals.is_empty(){
            ans.push('.');
        }
        if cycle{
            for i in 0..idx{
                ans.push(('0' as u8 + decimals[i] as u8) as char);
            }
            ans.push('(');
            for i in idx..decimals.len(){
                ans.push(('0' as u8 + decimals[i] as u8) as char);
            }
            ans.push(')');
        }else{
            for v in decimals{
                ans.push(('0' as u8 + v as u8) as char);
            }
        }

        ans

    }
}
// @lc code=end

#[test]
fn test(){
    let numerator = -1;
    let denominator = -2147483648;
    let ans = "0.0000000004656612873077392578125".to_string();
    let res = Solution::fraction_to_decimal(numerator, denominator);
    assert_eq!(res,ans);
}