use super::Solution;

/*
 * @lc app=leetcode.cn id=479 lang=rust
 *
 * [479] 最大回文数乘积
 */

// @lc code=start
impl Solution {
    pub fn largest_palindrome(n: i32) -> i32 {
        if n == 1{
            return 9;
        }
        let mut up = 10_u128.pow(n as u32) - 1;
        for i in (0..up).rev(){
            let mut p = i;
            let mut x = i;
            while x>0 {
                p = p*10 + x%10;
                x /=10;
            }
            x = up;
            while x*x >= p{
                if p%x == 0{
                    return (p%1337 )as i32;
                }
                x-=1;
            }
        }
    
        return 0;
    }
}
// @lc code=end

