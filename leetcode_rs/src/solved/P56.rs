use super::Solution;

/*
 * @lc app=leetcode.cn id=56 lang=rust
 *
 * [56] 合并区间
 */

// @lc code=start
use std::cmp::Ordering;
impl Solution {
    pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let len = intervals.len();
        if len <= 1{
            return intervals;
        }
        
        let mut intervals = intervals.clone();
        let mut ans = vec![];
        intervals.sort_by(|a,b|{
            match a[0].cmp(&b[0]){
                Ordering::Equal=>{
                    a[1].cmp(&b[1])
                },
                Ordering::Greater=>{
                    Ordering::Greater
                },
                Ordering::Less => Ordering::Less
            }
        });
        ans.push(intervals[0].clone());
        for i in intervals.iter().skip(1){
            if let Some(last) = ans.last_mut(){
                if last[1] >= i[0]{
                    last[1] = last[1].max(i[1]);
                }else{
                    ans.push(i.clone());
                }
            }
        }

        return ans;
    }
}
// @lc code=end

