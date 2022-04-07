use super::Solution;

/*
 * @lc app=leetcode.cn id=796 lang=rust
 *
 * [796] 旋转字符串
 */

// @lc code=start
impl Solution {
    pub fn rotate_string(s: String, goal: String) -> bool {

        if s.len()>goal.len(){
            return false;
        }

        let mut gs = goal.clone();
        gs.push_str(goal.as_str());

        if let Some(_) = gs.find(&s){
            return true;
        }else{
            return false;
        }

    }
}
// @lc code=end

