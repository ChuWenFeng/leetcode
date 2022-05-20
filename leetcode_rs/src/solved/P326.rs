use super::Solution;
/*
 * @lc app=leetcode.cn id=326 lang=rust
 *
 * [326] 3 的幂
 */

// @lc code=start
impl Solution {
    pub fn is_power_of_three(n: i32) -> bool {
        if n <= 0{
            return false;
        }
        let mut n = n;
        while n>1 {
            if n%3 != 0{
                return false;
            }
            n/=3;
        }

        if n != 1{
            return false;
        }

        true
    }
}
// @lc code=end

