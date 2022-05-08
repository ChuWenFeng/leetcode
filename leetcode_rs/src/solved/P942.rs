use super::Solution;
/*
 * @lc app=leetcode.cn id=942 lang=rust
 *
 * [942] 增减字符串匹配
 */

// @lc code=start
impl Solution {
    pub fn di_string_match(s: String) -> Vec<i32> {
        let len = s.len();
        let mut ans = vec![0;len+1];
        let mut max = len as i32;
        let mut min = 0;
        for (idx,c) in s.chars().enumerate(){
            if c == 'I'{
                ans[idx] = min;
                min+=1;
            }else{
                ans[idx] = max;
                max-=1;
            }
        }
        ans[len] = min;

        ans
    }
}
// @lc code=end

