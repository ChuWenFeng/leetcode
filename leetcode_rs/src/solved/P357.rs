use super::Solution;

/*
 * @lc app=leetcode.cn id=357 lang=rust
 *
 * [357] 计算各个位数不同的数字个数
 */

// @lc code=start
impl Solution {
    pub fn count_numbers_with_unique_digits(n: i32) -> i32 {
       
        match n {
            0 => return 1,
            1 => return 10,
            _ =>{
                let mut res = 10;
                let mut cur = 9;
                for i in 0..n-1{
                    cur *= (9-i);
                    res += cur;
                }
                return res;
            }
        }

        return 0;
    }
}
// @lc code=end

