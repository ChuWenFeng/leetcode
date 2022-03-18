use super::Solution;
/*
 * @lc app=leetcode.cn id=44 lang=rust
 *
 * [44] 通配符匹配
 */

// @lc code=start
impl Solution {
    pub fn is_match(s: String, p: String) -> bool {

        let s_len = s.len();
        let p_len = p.len();
        let mut dp = vec![vec![false;p_len+1];s_len+1];
        dp[0][0] = true;
        for (idx,char) in p.chars().enumerate(){
            if char == '*'{
                dp[0][idx+1] = true;
            }else{
                break;
            }
        }

        for (s_idx,s_char) in s.chars().enumerate(){
            for (p_idx,p_char) in p.chars().enumerate(){
                if p_char == '*'{
                    dp[s_idx+1][p_idx+1] = dp[s_idx+1][p_idx] || dp[s_idx][p_idx+1];
                }else if p_char == '?' || s_char == p_char{
                    dp[s_idx+1][p_idx+1] = dp[s_idx][p_idx];
                }
            }
        }
        return dp[s_len][p_len];
    }
}
// @lc code=end

#[test]
fn test(){
    let s = "aa".to_string();
    let p = "*".to_string();
    assert_eq!(Solution::is_match(s, p),true);
}