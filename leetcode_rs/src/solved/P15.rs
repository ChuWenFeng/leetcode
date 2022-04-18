use super::Solution;
/*
 * @lc app=leetcode.cn id=15 lang=rust
 *
 * [15] 三数之和
 */

// @lc code=start
impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        if nums.len()<=2{
            return vec![];
        }
        let mut nums = nums.clone();
        nums.sort();
        
        let mut ans = vec![];
        for i in 0..nums.len()-2{
            if i>0 && nums[i] == nums[i-1]{
                continue;
            }
            let mut k = nums.len()-1;
            for j in i+1..nums.len()-1{
                if j>i+1 && nums[j] == nums[j-1]{
                    continue;
                }
                let target = 0-nums[i]-nums[j];

                while j<k && nums[k]>target{
                    k-=1;
                }
                if j == k{
                    break;;
                }
                if nums[k] == target{
                    ans.push(vec![nums[i],nums[j],nums[k]])
                }
            }
        }
        ans
    }
}
// @lc code=end

#[test]
fn test(){
    let input = vec![-4,-2,-2,-2,0,1,2,2,2,3,3,4,4,6,6];
    let ans = vec![[-4,-2,6],[-4,0,4],[-4,1,3],[-4,2,2],[-2,-2,4],[-2,0,2]];
    let res = Solution::three_sum(input);
    assert_eq!(res,ans);
}