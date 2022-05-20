use super::Solution;
/*
 * @lc app=leetcode.cn id=436 lang=rust
 *
 * [436] 寻找右区间
 */

// @lc code=start
impl Solution {
    pub fn find_right_interval(intervals: Vec<Vec<i32>>) -> Vec<i32> {
        let mut list:Vec<(i32, i32, usize)> = intervals.iter().enumerate().map(|(i,v)|{
            (v[0],v[1],i)
        }).collect();
        let mut ans = vec![-1;intervals.len()];
        list.sort();
        for i in 0..list.len(){
            let cur = list[i];
            for j in i..list.len(){
                let check = list[j];
                if check.0 >= cur.1{
                    ans[cur.2] = check.2 as i32;
                    break;
                }
            }
        }

        ans
    }
}
// @lc code=end

