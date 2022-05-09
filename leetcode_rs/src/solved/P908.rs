use super::Solution;
/*
 * @lc app=leetcode.cn id=908 lang=rust
 *
 * [908] 最小差值 I
 */

// @lc code=start
impl Solution {
    pub fn smallest_range_i(nums: Vec<i32>, k: i32) -> i32 {
        let max = nums.iter().max().unwrap();
        let min = nums.iter().min().unwrap();
        0.max(max-min-k-k)

    }
}
// @lc code=end

