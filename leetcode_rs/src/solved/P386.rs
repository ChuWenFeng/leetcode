use super::Solution;

/*
 * @lc app=leetcode.cn id=386 lang=rust
 *
 * [386] 字典序排数
 */

// @lc code=start
impl Solution {
    pub fn lexical_order(n: i32) -> Vec<i32> {
        // let n = n as usize;
        let mut ans = vec![0;n as usize];
        let mut num = 1;
        for i in 0..n as usize{
            ans[i] = num;

            if num*10 <= n{
                num*=10;
            }else{
                while num%10 == 9 || num+1 > n{
                    num/=10;
                }
                num+=1;
            }
        }
        ans
    }
}
// @lc code=end

