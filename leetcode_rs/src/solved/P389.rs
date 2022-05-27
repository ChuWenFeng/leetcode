use super::Solution;
/*
 * @lc app=leetcode.cn id=389 lang=rust
 *
 * [389] 找不同
 */

// @lc code=start
impl Solution {
    pub fn find_the_difference(s: String, t: String) -> char {
        let mut count = vec![0;26];
        for c in t.chars(){
            count[c as usize - 'a' as usize]+=1;
        }
        for c in s.chars(){
            count[c as usize - 'a' as usize]-=1;
        }
        for (i,&v) in count.iter().enumerate(){
            if v > 0{
                return (i as u8 + 'a' as u8) as char ;
            }
        }
        return 'a';
    }
}
// @lc code=end

