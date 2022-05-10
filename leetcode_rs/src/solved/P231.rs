use super::Solution;

/*
 * @lc app=leetcode.cn id=231 lang=rust
 *
 * [231] 2 的幂
 */

// @lc code=start
impl Solution {
    pub fn is_power_of_two(n: i32) -> bool {
        if n == 1{
            return true;
        }
        if n <= 0{
            return false;
        }
        let mut n = n;
        while n > 1{
            if n&1 ==1{
                return false;
            }
            n/=2;
        }
        true
    }
}
// @lc code=end

#[test]
fn test(){
    let n = 16;
    let ans = true;

}