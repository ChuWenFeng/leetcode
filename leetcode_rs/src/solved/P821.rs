use super::Solution;

/*
 * @lc app=leetcode.cn id=821 lang=rust
 *
 * [821] 字符的最短距离
 */

// @lc code=start
impl Solution {
    pub fn shortest_to_char(s: String, c: char) -> Vec<i32> {
        let mut ans = vec![i32::MAX;s.len()];
        let clist:Vec<char> = s.chars().collect();
        let mut point = i32::MAX;
        for (idx,&sc) in clist.iter().enumerate(){
            if sc == c{
                point = idx as i32;
            }
            ans[idx] = (point - idx as i32).abs();
        }
        for (idx,&sc) in clist.iter().enumerate().rev(){
            if sc == c{
                point = idx as i32;
            }
            ans[idx] = (point - idx as i32).abs().min(ans[idx]);
        }

        ans
    }
}
// @lc code=end

