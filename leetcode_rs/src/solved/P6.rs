use super::Solution;
/*
 * @lc app=leetcode.cn id=6 lang=rust
 *
 * [6] Z 字形变换
 */

// @lc code=start
impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        if num_rows == 1{
            return s;
        }
        let mut ans = String::new();
        let mut list:Vec<Vec<char>> = vec![vec![];num_rows as usize];

        let mut idx = 0;
        let mut advance = 1;
        for c in s.chars(){
            if idx == 0{
                advance = 1;
            }
            if idx == num_rows -1{
                advance = -1;
            }
            list[idx as usize].push(c);

            idx+=advance;
        }
        for v in list{
            ans.extend(v.iter());
        }

        return ans;
    }
}
// @lc code=end

