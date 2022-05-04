use super::Solution;

/*
 * @lc app=leetcode.cn id=905 lang=rust
 *
 * [905] 按奇偶排序数组
 */

// @lc code=start
impl Solution {
    pub fn sort_array_by_parity(nums: Vec<i32>) -> Vec<i32> {
        let len = nums.len();
        let mut left = 0;
        let mut right = len-1;
        let mut nums = nums.clone();
        while left < right{
            while right>0 && nums[right]%2 == 1{
                right-=1;
            }
            while left<len && nums[left]%2 == 0{
                left+=1;
            }
            if left < right{
                let tmp = nums[left];
                nums[left] = nums[right];
                nums[right] = tmp;
            }
        }

        nums
    }
}
// @lc code=end

