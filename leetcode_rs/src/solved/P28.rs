use super::Solution;
/*
 * @lc app=leetcode.cn id=28 lang=rust
 *
 * [28] 实现 strStr()
 */

// @lc code=start
impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        let mut fail:Vec<usize> = vec![0;needle.len()];
        let mut nlist:Vec<char> = needle.chars().collect();
        for i in 1..needle.len(){
            let mut j = fail[i-1];
            while j > 0 && nlist[(j+1) as usize] != nlist[i]{
                j = fail[j as usize];
            }
            if nlist[(j+1)as usize] == nlist[i]{
                fail[i] = j+1;
            }
            
        }

        let mut best = 0;
        let mut ans = -1;
        for (idx,c) in haystack.chars().enumerate(){
            while best > 0 && nlist[(best+1) as usize] != c{
                best = fail[best as usize];
            }
            if nlist[(best+1) as usize] == c{
                best+=1;
            }
            if best == needle.len(){
                ans = (idx - best) as i32;
                break;
            }
        }

        ans 
    }
}
// @lc code=end

