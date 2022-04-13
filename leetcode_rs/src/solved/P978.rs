use super::Solution;
/*
 * @lc app=leetcode.cn id=978 lang=rust
 *
 * [978] 最长湍流子数组
 */

// @lc code=start
impl Solution {
    pub fn max_turbulence_size_1(arr: Vec<i32>) -> i32 {
        if arr.len()<= 1{
            return 1;
        }

        let mut left = 0;
        let mut ans = 1;

        let mut right = 0;
        while right < arr.len()-1{
            if left == right{
                if arr[left] == arr[left+1]{
                    left+=1;
                }
                right+=1;
            }else{
                if arr[right-1]<arr[right] && arr[right]> arr[right+1] {
                    right+=1;
                }else if arr[right-1]>arr[right] && arr[right]< arr[right+1] {
                    right+=1;
                }else{
                    left = right;
                }
            }
            ans = ans.max((right-left+1) as i32);
            
        }

        return ans;
    }

    pub fn max_turbulence_size(arr: Vec<i32>) -> i32 {
        let len = arr.len();
        let mut ans = 1;
        let mut dp= (1,1);

        for i in 1..len{
            if arr[i-1]<arr[i]{
                dp.0 = dp.1+1;
                dp.1 = 1;
            }else if arr[i-1] >arr[i] {
                dp.1 = dp.0+1;
                dp.0 = 1;
            }else{
                dp = (1,1);
            }
            ans = ans.max(dp.0.max(dp.1));
        }

        return ans;
    }
}
// @lc code=end

