use super::Solution;
/*
 * @lc app=leetcode.cn id=1004 lang=rust
 *
 * [1004] 最大连续1的个数 III
 */

// @lc code=start
impl Solution {
    pub fn longest_ones(nums: Vec<i32>, k: i32) -> i32 {
        let len = nums.len();
        if len <= k as usize{
            return len as i32;
        }
        let mut ans = 0;
        let mut diffnum = 0;
        let mut  left = 0;
        for i in 0..len{
            if nums[i] != 1{
                diffnum +=1;
            }
            while diffnum > k{
                if nums[left] != 1{
                    diffnum-=1;
                }
                left +=1;
            }

            if i - left +1 > ans{
                ans = i-left+1;
            }
        }
        return ans as i32;
    }
}
// @lc code=end

