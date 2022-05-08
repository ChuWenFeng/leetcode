use super::Solution;
/*
 * @lc app=leetcode.cn id=442 lang=rust
 *
 * [442] 数组中重复的数据
 */

// @lc code=start
impl Solution {
    pub fn find_duplicates_1(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums.clone();
        let mut ans = vec![];

        for i in 0..nums.len(){
            let mut v = nums[i] as usize;
            while nums[i] != nums[v -1]{
                let tmp = nums[i];
                nums[i] = nums[v -1];
                nums[v -1] = tmp;
                v = nums[i] as usize;
            }
        }

        for (idx,&v) in nums.iter().enumerate(){
            if v as usize-1 != idx{
                ans.push(v);
            }
        }
        ans
    }
    pub fn find_duplicates(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums.clone();
        let mut ans = vec![];

        for i in nums.iter(){
            let t = nums[*i as usize -1];
            if t < 0 {
                ans.push(t as i32 +1);
            }else{
                // nums[*i as usize -1] = -t;
            }
        }

        ans;
        unimplemented!()
    }
}
// @lc code=end

#[test]
fn test(){
    let nums = vec![4,3,2,7,8,2,3,1];
    let ans = vec![2,3];
    let res = Solution::find_duplicates(nums);
    assert_eq!(res,ans);
}