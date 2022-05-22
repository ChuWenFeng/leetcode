use super::Solution;
/*
 * @lc app=leetcode.cn id=464 lang=rust
 *
 * [464] 我能赢吗
 */

// @lc code=start
impl Solution {
    pub fn can_i_win(max_choosable_integer: i32, desired_total: i32) -> bool {

        if ((1+max_choosable_integer)*max_choosable_integer)/2 < desired_total{
            return false;
        }

        let mut dp = vec![-1;1<<max_choosable_integer];



        return dfs(0,0,&mut dp,max_choosable_integer,desired_total) == 1
    }
}
fn dfs(useNum:usize,curTot:i32,dp:&mut Vec<i8>,max_choosable_integer:i32,desired_total:i32)->i8{
    let mut dv = dp[useNum];
    if dv != -1{
        return dv;
    }
    for i in 0..max_choosable_integer{
        if useNum>>i&1 == 0 && (curTot+i+1 >= desired_total || dfs(useNum|1<<i, curTot+i+1, dp, max_choosable_integer, desired_total)==0 ){
            dp[useNum] = 1;
            return 1;
        }
    }
    dp[useNum] = 0;
    return 0;
}
// @lc code=end

#[test]
fn test(){
    let max_choosable_integer = 10;
    let desired_total = 11;
    let ans = false;
    let res = Solution::can_i_win(max_choosable_integer, desired_total);
    assert_eq!(res,ans);
}