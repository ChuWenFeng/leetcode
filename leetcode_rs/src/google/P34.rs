use super::Solution;
/*
 * @lc app=leetcode.cn id=34 lang=rust
 *
 * [34] 在排序数组中查找元素的第一个和最后一个位置
 */

// @lc code=start
impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut ans = vec![-1,-1];
        let len  = nums.len();
        if nums.len() == 0{
            return ans;
        }
        let mut left = 0;
        let mut right = nums.len();
        let mut pre = nums[0];
        if pre == target{
            ans[0] = 0;
        }
        if nums[len-1] == target{
            ans[1] = (len-1) as i32;
        }

        let mut mid = (left+right)/2;

        while left<right{
            let mid = (left+right)/2;
            if nums[mid] == target{

            }
        }
        
        return ans;
    }
}
// @lc code=end
#[test]
fn test(){
    let result = Solution::search_range(vec![1], 0);
    assert_eq!(result,vec![-1,-1]);
}
