use super::Solution;
/*
 * @lc app=leetcode.cn id=330 lang=rust
 *
 * [330] 按要求补齐数组
 */

// @lc code=start
impl Solution {
    pub fn min_patches(nums: Vec<i32>, n: i32) -> i32 {
        let mut i = 0;
        let mut x = 1;
        let mut ans = 0;
        while x<=n{
            if i < nums.len() && nums[i] <= x{
                x+=nums[i];
                i+=1;
            }else{
                x*=2;
                ans+=1;
            }
        }
        ans
    }
}
// @lc code=end

