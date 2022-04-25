use super::Solution;
/*
 * @lc app=leetcode.cn id=868 lang=rust
 *
 * [868] 二进制间距
 */

// @lc code=start
impl Solution {
    pub fn binary_gap(n: i32) -> i32 {
        let mut n = n;
        let mut ans = 0;
        let mut count = 0;
        while n>0 && n & 1 != 1{
            n = n>>1;
        }
        while n>1{
            n>>=1;
            count+=1;
            while n>0 && n & 1 != 1{
                n = n>>1;
                count+=1;
            }
            ans = ans.max(count);
            count = 0;

        }
        ans
    }
}
// @lc code=end

#[test]
fn test(){
    let n = 8;
    let ans = 0;
    let res = Solution::binary_gap(n);
    assert_eq!(res,ans);
}