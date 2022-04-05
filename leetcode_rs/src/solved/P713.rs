use super::Solution;
/*
 * @lc app=leetcode.cn id=713 lang=rust
 *
 * [713] 乘积小于K的子数组
 */

// @lc code=start
impl Solution {
    pub fn num_subarray_product_less_than_k(nums: Vec<i32>, k: i32) -> i32 {
        if k <= 1{
            return 0;
        }
        let mut ans = 0;
        let mut left = 0;
        let mut mulit = 1;
        for (right,&val) in nums.iter().enumerate(){
            mulit *= val;
            while mulit >= k{
                mulit /= nums[left];
                left+=1;
            }
            ans += right - left + 1;
        }

        ans as i32
    }
}
// @lc code=end

