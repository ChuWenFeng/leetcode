use std::num;

use super::Solution;
/*
 * @lc app=leetcode.cn id=313 lang=rust
 *
 * [313] 超级丑数
 */

// @lc code=start
impl Solution {
    pub fn nth_super_ugly_number(n: i32, primes: Vec<i32>) -> i32 {
        let mut dp = vec![0;n as usize+1];
        let m = primes.len();
        let mut pointers = vec![0;m];
        let mut nums = vec![1;m];
        for i in 1..=n as usize{
            let mut min = i32::MAX;
            for j in 0..m{
                min = min.min(nums[j]);
            }
            dp[i] = min;
            for j in 0..m{
                if nums[j] == min{
                    pointers[j]+=1;
                    nums[j] = dp[pointers[j] as usize]*primes[j];
                }
            }

        }
        return dp[n as usize];
    }
}
// @lc code=end

