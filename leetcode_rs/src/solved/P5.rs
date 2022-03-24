use super::Solution;

/*
 * @lc app=leetcode.cn id=5 lang=rust
 *
 * [5] 最长回文子串
 */

// @lc code=start
impl Solution {
    // pub fn longest_palindrome_dp(s: String) -> String {
    //     let len = s.len();
    //     if len < 2{
    //         return s;
    //     }
    //     let sc:Vec<char> = s.chars().collect();
    //     let mut maxlen = 1;
    //     let mut begin = 0;
    //     let mut dp = vec![vec![false;len];len];
    //     for i in 0..len{
    //         dp[i][i] = true;
    //         dp[len-1][len-1] = true;
    //     }
    //     for l in 2..=len{
    //         for i in 0..len-l+1{
    //             let j = i+l-1;
    //             if sc[i] != sc[j]{
    //                 dp[i][j] = false;
    //             }else{
    //                 if l == 2{
    //                     dp[i][j] = true;
    //                 }else{
    //                     dp[i][j] = dp[i+1][j-1];
    //                 }
    //             }
    //             if dp[i][j] && l > maxlen{
    //                 maxlen= l;
    //                 begin = i;
    //             }
    //         }
    //     }
    //     let ans = s[begin..begin+maxlen].into();
    //     return ans;
    // }

    pub fn longest_palindrome(s:String) -> String{
        let mut ml = 1;
        let mut begin = 0;
        let len = s.len();
        if len < 2{
            return s;
        }
        // let mut dp = vec![vec![false;len];len];
        let sc:Vec<char> = s.chars().collect();
        // for i in 0..len{
        //     dp[i][i] = true;
        // }

        for i in 0..len-1{
            let mut minlen = if i < len-i-1{i}else{len-i-1};
            for j in 0..=minlen{
                if sc[i-j] != sc[i+j]{
                    break;
                }else{
                    if j*2+1 > ml{
                        ml = j*2+1;
                        begin = i-j;
                    }
                }
            }

            minlen = if i < len-i-2{i}else{len-i-2};
            if sc[i] == sc[i+1]{
                for j in 0..=minlen{
                    if sc[i-j] != sc[i+1+j]{
                        break;
                    }else{
                        if j*2+2 > ml{
                            ml = j*2+2;
                            begin = i-j;
                        }
                    }
                }
            }
        }
        return s[begin..begin+ml].into();
    }

}
// @lc code=end

#[test]
fn test(){
    let input = "cbba".to_string();
    let ans = "bb".to_string();

    let res = Solution::longest_palindrome(input);
    assert_eq!(res,ans);
}

