use super::Solution;

/*
 * @lc app=leetcode.cn id=66 lang=rust
 *
 * [66] 加一
 */

// @lc code=start
impl Solution {
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        let mut digits = digits.clone();
        let mut carry = 1;
        for v in digits.iter_mut().rev(){
            *v+=carry;
            carry = *v / 10;
            *v%=10;
        }
        if carry != 0{
            let mut tmp = vec![carry];
            tmp.extend(digits.iter());
            digits = tmp;
        }
        digits
    }
}
// @lc code=end

