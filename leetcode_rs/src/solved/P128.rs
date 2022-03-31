use super::Solution;

/*
 * @lc app=leetcode.cn id=128 lang=rust
 *
 * [128] 最长连续序列
 */

// @lc code=start
impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let mut ans = 0;
        let mut numset = std::collections::HashSet::new();
        for i in nums.iter(){
            numset.insert(i);
        }
        for &i in numset.iter(){
            let mut currentlen = 1;
            if let Some(_) = numset.get(&(i-1)){
                continue;
            }
            while let Some(_) = numset.get(&(i+currentlen)){
                currentlen+=1;
            }
            if currentlen>ans{
                ans = currentlen;
            }

        }
        return ans;
    }
}
// @lc code=end

