use super::Solution;
/*
 * @lc app=leetcode.cn id=368 lang=rust
 *
 * [368] 最大整除子集
 */

// @lc code=start
impl Solution {
    pub fn largest_divisible_subset(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums.clone();
        nums.sort();
        let mut dp = vec![1;nums.len()];
        let mut ans = vec![];
        let mut maxsize = 1;
        for j in 1..nums.len(){
            for i in 0..j{
                if nums[j] % nums[i] == 0{
                    dp[j] = dp[j].max(dp[i]+1);
                }
            }
            maxsize = maxsize.max(dp[j]);
        }

        while maxsize>0{
            for (i,&v) in dp.iter().enumerate().rev(){
                if v == maxsize{
                    if !ans.is_empty(){
                        if ans.last().unwrap() % nums[i] == 0{
                            ans.push(nums[i]);
                            maxsize-=1;
                            break;
                        }
                    }else{
                        ans.push(nums[i]);
                        maxsize-=1;
                        break;
                    }
                    
                }
            }
        }

        ans
    }
}
// @lc code=end

#[test]
fn test(){
    let nums = vec![1,2,4,8];
    let ans = vec![1,2,4,8];
    let res = Solution::largest_divisible_subset(nums);
    assert_eq!(res,ans);
}