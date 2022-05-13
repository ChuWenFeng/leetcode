use super::Solution;
/*
 * @lc app=leetcode.cn id=228 lang=rust
 *
 * [228] 汇总区间
 */

// @lc code=start
impl Solution {
    pub fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
        let mut ans = vec![];
        if nums.len()== 0{
            return ans;
        }
        
        let mut left = 0;
        for i in 1..nums.len(){
            if nums[i]-nums[i-1] != 1{
                if i-1 > left{
                    ans.push(get_ranges(nums[left],nums[i-1]));
                }else{
                    ans.push(nums[left].to_string());
                }
                
                left = i;
            }
        }
        if nums.len()-1 > left{
            ans.push(get_ranges(nums[left],nums[nums.len()-1]));
        }else{
            ans.push(nums[left].to_string());
        }

        ans
    }
}

fn get_ranges(left:i32,right:i32)->String{
    return format!("{}->{}",left,right);
}
// @lc code=end

