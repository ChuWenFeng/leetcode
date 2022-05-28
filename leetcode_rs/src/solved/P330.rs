use super::Solution;
/*
 * @lc app=leetcode.cn id=330 lang=rust
 *
 * [330] 按要求补齐数组
 */

// @lc code=start
impl Solution {
    pub fn min_patches(nums: Vec<i32>, n: i32) -> i32 {
        let mut i = 0;
        let mut x:i64 = 1;
        let mut ans = 0;
        let mut n = n as i64;
        while x<=n{
            if i < nums.len() && nums[i] as i64 <= x{
                x+=nums[i] as i64;
                i+=1;
            }else{
                x*=2;
                ans+=1;
            }
        }
        ans
    }
}
// @lc code=end

#[test]
fn test(){
    let nums = vec![1,2,31,33];
    let n = 2147483647;
    let mut ans = 28;
    let mut res = Solution::min_patches(nums, n);
    assert_eq!(res,ans);
}