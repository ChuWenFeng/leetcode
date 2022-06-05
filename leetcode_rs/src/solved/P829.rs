use super::Solution;
/*
 * @lc app=leetcode.cn id=829 lang=rust
 *
 * [829] 连续整数求和
 */

// @lc code=start
use std::cmp::Ordering;
impl Solution {
    pub fn consecutive_numbers_sum(n: i32) -> i32 {
        let mut ans = 0;
        let n = n*2;
        let mut k = 1;
        while k*k < n{
            
            if n %k != 0{
                k+=1;
                continue;
            }
            if (n/k-(k-1))%2 == 0{
                ans+=1;
            }
            k+=1;
        }

        ans
    }
}
// @lc code=end

#[test]
fn test(){
    let n = 9;
    let ans = 3;
    let res = Solution::consecutive_numbers_sum(n);
    assert_eq!(res,ans);
}