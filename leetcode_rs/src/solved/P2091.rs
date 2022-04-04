use super::Solution;
/*
 * @lc app=leetcode.cn id=2091 lang=rust
 *
 * [2091] 从数组中移除最大值和最小值
 */

// @lc code=start
impl Solution {
    pub fn minimum_deletions(nums: Vec<i32>) -> i32 {
        let len = nums.len();
        let mut maxidx = 0;
        let mut max = i32::MIN;
        let mut minidx = 0;
        let mut min = i32::MAX;
        for (idx,&val) in nums.iter().enumerate(){
            if val > max{
                max = val;
                maxidx = idx;
            }
            if val < min{
                min = val;
                minidx = idx;
            }
        }

        let a1 = minidx.max(maxidx)+1;
        let a2 = if minidx > maxidx{
            maxidx+1 + len - minidx
        }else{
            minidx+1 + len - maxidx
        };
        let a3 = len - minidx.min(maxidx);


        return a1.min(a2).min(a3) as i32;
    }
}
// @lc code=end

