use super::Solution;
/*
 * @lc app=leetcode.cn id=413 lang=rust
 *
 * [413] 等差数列划分
 */

// @lc code=start
impl Solution {
    pub fn number_of_arithmetic_slices(nums: Vec<i32>) -> i32 {
        let len = nums.len();
        if len <3{
            return 0;
        }
        let mut dp = vec![0;len];
        let mut d = nums[1]-nums[0];
        for (i,v) in nums.iter().enumerate().skip(2){
            let currd = nums[i] - nums[i-1];
            if currd == d{
                dp[i] = dp[i-1]+1;
            }else{
                dp[i] = 0;
                d = currd;
            }
        }

        dp.iter().sum()
    }
}
// @lc code=end

