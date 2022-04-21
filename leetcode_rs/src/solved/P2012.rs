use core::num;

use super::Solution;

/*
 * @lc app=leetcode.cn id=2012 lang=rust
 *
 * [2012] 数组美丽值求和
 */

// @lc code=start
impl Solution {
    pub fn sum_of_beauties(nums: Vec<i32>) -> i32 {
        let mut ans = 0;
        let len = nums.len();

        let mut preMax = vec![0;len];
        let mut tailMin = vec![0;len];
        let mut max = 0;
        let mut min = i32::MAX;
        for (idx,v) in nums.iter().enumerate(){
            max = max.max(*v);
            preMax[idx] = max;
        }
        for (idx,v) in nums.iter().enumerate().rev(){
            min = min.min(*v);
            tailMin[idx] = min;
        }

        for i in 1..len-1{
            if preMax[i-1]<nums[i] && nums[i]<tailMin[i+1]{
                ans+=2;
            }else if nums[i-1]<nums[i] && nums[i] < nums[i+1]{
                ans+=1;
            }
        }

        ans
    }
}
// @lc code=end

#[test]
fn test(){
    let nums = vec![2,4,6,4];
    let ans = 1;
    let res = Solution::sum_of_beauties(nums);
    assert_eq!(res,ans);
}