use super::Solution;
/*
 * @lc app=leetcode.cn id=42 lang=rust
 *
 * [42] 接雨水
 */

// @lc code=start
impl Solution {
    pub fn trap1(height: Vec<i32>) -> i32 {
        let len = height.len();
        let mut ans = 0;
        let mut leftMax = Vec::new();
        let mut rightMax = Vec::new();
        leftMax.resize(len, 0);
        rightMax.resize(len, 0);
        leftMax[0] = height[0];
        rightMax[len-1] = height[len-1];
        for (idx,&val) in height.iter().enumerate().skip(1){
            if leftMax[idx-1] < val{
                leftMax[idx] = val;
            }else{
                leftMax[idx] = leftMax[idx-1];
            }
        }
        for (idx,&val) in height.iter().enumerate().rev().skip(1){
            if rightMax[idx+1]<val{
                rightMax[idx] = val;
            }else{
                rightMax[idx]= rightMax[idx+1];
            }
        }
    
        for (idx,&val) in height.iter().enumerate(){
            if leftMax[idx] < rightMax[idx]{
                ans += leftMax[idx]-val;
            }else{
                ans += rightMax[idx]-val;
            }
        }
    
        return ans;
    }

    pub fn trap(height: Vec<i32>) -> i32 {
        let len = height.len();
        let mut ans = 0;
        let  (mut left,mut right,mut leftMax,mut rightMax) = (0,len-1,0,0);
        while left < right{
            if leftMax < height[left]{
                leftMax = height[left]
            }
            if rightMax < height[right]{
                rightMax = height[right]
            }
            if height[left] < height[right]{
                ans += leftMax - height[left];
                left+=1;
            }else{
                ans += rightMax - height[right];
                right-=1;
            }
        }

        return ans;
    }

}
// @lc code=end

#[test]
fn test(){
    let input = vec![0,1,0,2,1,0,1,3,2,1,2,1];
    let ans = Solution::trap(input);
    assert_eq!(ans,6);
}