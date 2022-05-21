use super::Solution;
/*
 * @lc app=leetcode.cn id=961 lang=rust
 *
 * [961] 在长度 2N 的数组中找出重复 N 次的元素
 */

// @lc code=start
impl Solution {
    pub fn repeated_n_times(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        for gap in 1..=3{
            let mut j = 0;
            while j+gap < n{
                if nums[j] == nums[j+gap]{
                    return nums[j];
                }
                j+=1;
            }
        }
        return -1;
    }
}
// @lc code=end

