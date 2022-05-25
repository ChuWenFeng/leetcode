use super::Solution;
/*
 * @lc app=leetcode.cn id=467 lang=rust
 *
 * [467] 环绕字符串中唯一的子字符串
 */

// @lc code=start
impl Solution {
    pub fn find_substring_in_wrapround_string(p: String) -> i32 {
        
        let len = p.len();
        let mut dp = vec![0;26];
        
        let mut prec = 'a';
        let mut k = 0;
        for c in p.chars(){
            let mut idx = c as usize - 'a' as usize;
            let mut preidx = (prec as usize - 'a' as usize + 26)%26;
            if idx as usize % 26 == (preidx as usize +1)%26{
                k+=1;
            }else{
               k=1;
            }
            dp[idx] = dp[idx].max(k);
            prec = c;
        }

        dp.iter().sum()
    }
}
// @lc code=end

#[test]
fn test(){
    let p = "cdefghefghijklmnopqrstuvwxmnijklmnopqrstuvbcdefghijklmnopqrstuvwabcddefghijklfghijklmabcdefghijklmnopqrstuvwxymnopqrstuvwxyz"
    .into();
    let ans = 339;
    let res = Solution::find_substring_in_wrapround_string(p);
    assert_eq!(res,ans);
}