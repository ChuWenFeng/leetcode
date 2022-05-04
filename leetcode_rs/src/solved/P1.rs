use super::Solution;
/*
 * @lc app=leetcode.cn id=1 lang=rust
 *
 * [1] 两数之和
 */

// @lc code=start
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut ans = vec![];
        let len = nums.len();

        for i in 0..len{
            for j in i+1..len{
                if nums[i]+nums[j] == target{
                    return vec![i as i32,j as i32];
                }
            }
        }

        ans
    }
}
// @lc code=end

