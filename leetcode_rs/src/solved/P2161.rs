use super::Solution;
/*
 * @lc app=leetcode.cn id=2161 lang=rust
 *
 * [2161] 根据给定数字划分数组
 */

// @lc code=start
impl Solution {
    pub fn pivot_array(nums: Vec<i32>, pivot: i32) -> Vec<i32> {
        let len = nums.len();
        let mut ans = vec![pivot;len];

        let mut left = 0;
        let mut right = len-1;
        for v in nums{
            if v < pivot{
                ans[left] = v;
                left+=1;
            }
            if v > pivot{
                ans[right] = v;
                right-=1;
            }
        }
        left = right+1;
        right = len-1;
        while left<right{
            let tmp = ans[left];
            ans[left] = ans[right];
            ans[right] = tmp;
            left+=1;
            right-=1;
        }

        ans
    }
}
// @lc code=end

