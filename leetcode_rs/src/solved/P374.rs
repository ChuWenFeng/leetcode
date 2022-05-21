use super::Solution;
unsafe fn guess(num: i32) -> i32 {0}
/*
 * @lc app=leetcode.cn id=374 lang=rust
 *
 * [374] 猜数字大小
 */

// @lc code=start
/** 
 * Forward declaration of guess API.
 * @param  num   your guess
 * @return 	     -1 if num is lower than the guess number
 *			      1 if num is higher than the guess number
 *               otherwise return 0
 * unsafe fn guess(num: i32) -> i32 {}
 */

impl Solution {
    unsafe fn guessNumber(n: i32) -> i32 {
        let mut left = 1;
        let mut right = n;
        loop {
            match guess(left + (right-left)/2) {
                -1=>{
                    right = left + (right-left)/2;
                    
                },
                0=>{
                    return left + (right-left)/2;
                },
                1=>{
                    left = left + (right-left)/2 +1;
                },
                _=>{}
            }
        }
        return left + (right-left)/2;
    }
}
// @lc code=end

