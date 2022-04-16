use super::Solution;

/*
 * @lc app=leetcode.cn id=1641 lang=rust
 *
 * [1641] 统计字典序元音字符串的数目
 */

// @lc code=start
impl Solution {
    pub fn count_vowel_strings(n: i32) -> i32 {
        let mut dp = [1,1,1,1,1];

        for _ in 1..n{
            let mut tmp = [0,0,0,0,0];
            for i in (0..5).rev(){
                tmp[i] = dp[i..5].iter().sum();
            }
            dp = tmp;
        }

        let mut ans = dp.iter().sum();
        return ans;
    }
}
// @lc code=end

