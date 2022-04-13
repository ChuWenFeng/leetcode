use super::Solution;
/*
 * @lc app=leetcode.cn id=806 lang=rust
 *
 * [806] 写字符串需要的行数
 */

// @lc code=start
impl Solution {
    pub fn number_of_lines(widths: Vec<i32>, s: String) -> Vec<i32> {
        let mut line_count = 1;
        let mut last_width = 0;
        for i in s.chars(){
            let idx = i as usize - 'a' as usize;
            last_width+=widths[idx];
            if last_width >100{
                last_width = widths[idx];
                line_count+=1;
            }
        }

        return vec![line_count,last_width];
    }
}
// @lc code=end

