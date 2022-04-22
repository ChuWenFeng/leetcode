use super::Solution;

/*
 * @lc app=leetcode.cn id=396 lang=rust
 *
 * [396] 旋转函数
 */

// @lc code=start
impl Solution {
    pub fn max_rotate_function(nums: Vec<i32>) -> i32 {
        let sum:i32 = nums.iter().sum();
        let len = nums.len() as i32;
        let mut ans = 0;
        let mut f = nums.iter().enumerate().fold(0, |acc,(idx,x)|{
            acc+idx as i32 * x
        });
        ans = f;
        for &v in nums.iter().rev(){
            f = f - (len * v) + sum;
            ans = ans.max(f);
        }

        ans
    }
}
// @lc code=end

#[test]
fn test(){
    let nums = vec![4,3,2,6];
    let ans = 26;
    let res = Solution::max_rotate_function(nums);
    assert_eq!(res,ans);
}