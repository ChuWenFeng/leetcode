use std::num;

use super::Solution;
/*
 * @lc app=leetcode.cn id=162 lang=rust
 *
 * [162] 寻找峰值
 */

// @lc code=start
impl Solution {
    pub fn find_peak_element_(nums: Vec<i32>) -> i32 {
        let len = nums.len();
        if len <= 1{
            return 0;
        }
        if nums[0]> nums[1]{
            return 0;
        }
        if nums[len-1]>nums[len-2]{
            return len as i32 -1;
        }
        for i in 1..len-1{
            if nums[i]>nums[i-1] && nums[i]> nums[i+1]{
                return i as i32;
            }
        }
        0
    }

    //二分查找 O(logn)
    pub fn find_peak_element(nums: Vec<i32>) -> i32 {
        let len = nums.len();
        let get = |i:i32|{
            if i == -1 || i == len as i32{
                return i32::MIN;
            }
            return nums[i as usize];
        };

        let mut left = 0;
        let mut right = len as i32 -1;

        while left<=right{
            let mid = (left+right)/2;
            if get(mid-1)<get(mid) && get(mid+1)<get(mid){
                return mid;
            }
            if get(mid)<get(mid+1){
                left = mid+1;
            }else{
                right = mid -1;
            }
        }
        0
    }
}
// @lc code=end

