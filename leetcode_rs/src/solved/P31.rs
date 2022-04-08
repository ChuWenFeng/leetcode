use super::Solution;

/*
 * @lc app=leetcode.cn id=31 lang=rust
 *
 * [31] 下一个排列
 */

// @lc code=start
impl Solution {
    pub fn next_permutation(nums: &mut Vec<i32>) {
        let len = nums.len();
        if len <=1{
            return;
        }
        let mut idx = len-2;

        while idx>0 && nums[idx]>=nums[idx+1]{
            idx-=1;
        }
        let mut flag = false;
        for i in (idx+1..len).rev(){
            if nums[i]>nums[idx]{
                flag = true;
                let tmp = nums[i];
                nums[i] = nums[idx];
                nums[idx] = tmp;
                break;
            }
        }
        if flag{
            nums[idx+1..len].reverse();
        }else{
            nums.reverse();
        }
        
    }
}
// @lc code=end

