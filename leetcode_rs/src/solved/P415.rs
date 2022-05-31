use super::Solution;
/*
 * @lc app=leetcode.cn id=415 lang=rust
 *
 * [415] 字符串相加
 */

// @lc code=start
impl Solution {
    pub fn add_strings(num1: String, num2: String) -> String {
        let n = num1.len();
        let m = num2.len();
        let mut ans = String::new();
        let mut num1 = num1;
        let mut num2 = num2;
        if n < m{
            let tmp = num1;
            num1 = num2;
            num2 = tmp;
        }

        let mut iter1 = num1.chars().rev();
        let mut iter2 = num2.chars().rev();
        let mut carry = 0;
        for i in 0..n.min(m){
            let c1 = iter1.next().unwrap();
            let c2 = iter2.next().unwrap();
            let n1 = c1 as i32 - '0' as i32;
            let n2 = c2 as i32 - '0' as i32;
            let mut n = n1+n2+carry;
            carry = n/10;
            n = n%10;

            ans.push((n as u8 + '0' as u8) as char);
        }

        for c in iter1{
            let mut n = c as i32 - '0' as i32;
            n+=carry;
            carry = n/10;
            n%=10;
            ans.push((n as u8 + '0' as u8) as char);
        }
        if carry>0{
            ans.push((carry as u8 + '0' as u8) as char);
        }

        ans.chars().rev().collect()
    }
}
// @lc code=end

