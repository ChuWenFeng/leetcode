use super::Solution;
/*
 * @lc app=leetcode.cn id=6009 lang=rust
 *
 * [6009] 使两字符串互为字母异位词的最少步骤数
 */

// @lc code=start
impl Solution {
    pub fn min_steps(s: String, t: String) -> i32 {
        let mut ans = 0;

        let mut s_count = vec![0;26];
        let mut t_count = vec![0;26];

        for c in s.chars(){
            s_count[(c as usize) - 97]+=1;
        }
        for c in t.chars(){
            t_count[c as usize - 97]+=1;
        }
        for (&sc,&tc) in s_count.iter().zip(t_count.iter()){
            if sc > tc{
                ans += sc - tc;
            }else{
                ans += tc - sc;
            }
        }

        return ans;
    }
}
// @lc code=end

