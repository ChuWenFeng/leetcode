use super::Solution;
/*
 * @lc app=leetcode.cn id=1461 lang=rust
 *
 * [1461] 检查一个字符串是否包含所有长度为 K 的二进制子串
 */

// @lc code=start
use std::collections::HashSet;
impl Solution {
    pub fn has_all_codes(s: String, k: i32) -> bool {
        let expect:usize = 2_usize.pow(k as u32);
        let len = s.len();
        let k = k as usize;
        if len  + 1 < expect+k{
            return false;
        }
        let mut hashset:HashSet<String> = HashSet::new();
        
        let mut idx = 0;
        while idx <= len - k {
            hashset.insert(s[idx..idx+k].to_string());
            idx+=1;
        }
        let mut ans = 0;
        for _ in hashset.iter(){
            ans+=1;
        }
        if ans != expect{
            return false;
        }

        return true;
    }
}
// @lc code=end

#[test]
fn test(){
    let s = "0".to_string();
    let k = 20;
    let ans = true;
    let res = Solution::has_all_codes(s, k);
    assert_eq!(res,ans);
}