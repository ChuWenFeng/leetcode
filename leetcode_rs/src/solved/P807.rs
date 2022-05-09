use super::Solution;
/*
 * @lc app=leetcode.cn id=807 lang=rust
 *
 * [807] 保持城市天际线
 */

// @lc code=start
impl Solution {
    pub fn max_increase_keeping_skyline(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let mut ans = 0;
        let mut rowmax = vec![0;n];
        let mut colmax = vec![0;n];

        for (i,row) in grid.iter().enumerate(){
            let mut rmax = 0;
            for (j,&v) in row.iter().enumerate(){
                rmax = rmax.max(v);
                colmax[j] = colmax[j].max(v);
            }
            rowmax[i] = rmax;
        }

        for (i,row) in grid.iter().enumerate(){
            for (j,&v) in row.iter().enumerate(){
                ans += rowmax[i].min(colmax[j]) - v;
            }
        }


        ans
    }
}
// @lc code=end

