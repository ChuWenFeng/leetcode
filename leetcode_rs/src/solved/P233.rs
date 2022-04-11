use super::Solution;

/*
 * @lc app=leetcode.cn id=233 lang=rust
 *
 * [233] 数字 1 的个数
 */

// @lc code=start
impl Solution {
    pub fn count_digit_one(n: i32) -> i32 {
        
        let mut count = 0;
       
        let mut n = n;
        let mut digit = 1;

        while n >= digit{
            let d = digit * 10;
            count += (n/d)*digit;
            let nmod = n % d;
            count += 0.max(nmod-digit+1).min(digit);

            digit *=10;
        }
        

        return count;
    }
}
// @lc code=end

#[test]
fn test(){
    let input = 13;
    let ans = 6;
    let res = Solution::count_digit_one(input);
    assert_eq!(res,ans);
}