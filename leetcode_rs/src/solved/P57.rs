use super::Solution;
/*
 * @lc app=leetcode.cn id=57 lang=rust
 *
 * [57] 插入区间
 */

// @lc code=start
impl Solution {
    pub fn insert(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        if intervals.is_empty(){
            return vec![new_interval];
        }
        let mut ans = vec![];
        let len = intervals.len();
        let mut left = intervals.len();
        let mut right = -1;
        for (idx,v) in intervals.iter().enumerate(){
            if v[1]>=new_interval[0]{
                left = idx;
                break;
            }
        }
        for (idx,v) in intervals.iter().enumerate().rev(){
            if v[0]<=new_interval[1]{
                right = idx as i32;
                break;
            }
        }



        for i in 0..left{
            ans.push(intervals[i].clone());
        }
        if left < len && right >= 0{
            ans.push(vec![intervals[left][0].min(new_interval[0]),intervals[right as usize][1].max(new_interval[1])]);
        }else{
            ans.push(new_interval);
        }
        for i in (right+1) as usize..intervals.len(){
            ans.push(intervals[i].clone());
        }
        ans
    }
}
// @lc code=end

#[test]
fn test(){
    let intervals:Vec<Vec<i32>> = [[6,8]].iter().map(|x|vec![x[0],x[1]]).collect();
    let new_interval = vec![1,5];
    let ans:Vec<Vec<i32>> = [[1,5],[6,8]].iter().map(|x|vec![x[0],x[1]]).collect();
    let res = Solution::insert(intervals, new_interval);
    assert_eq!(res,ans);
}