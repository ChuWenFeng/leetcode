use super::Solution;
/*
 * @lc app=leetcode.cn id=883 lang=rust
 *
 * [883] 三维形体投影面积
 */

// @lc code=start
impl Solution {
    pub fn projection_area(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let mut rowlist:Vec<i32> = vec![0;n];
        let mut collist:Vec<i32> = vec![0;n];
        let mut button = 0;

        for (i,row) in grid.iter().enumerate(){
            let mut rmax = 0;
            for (j,&col) in row.iter().enumerate(){
                rmax = rmax.max(col);
                collist[j] = collist[j].max(col);
                if col > 0{
                    button+=1;
                }
            }
            rowlist[i] = rmax;
        }
        let rs:i32  = rowlist.iter().sum();
        let cs:i32 = collist.iter().sum();
        button + rs + cs

    }
}
// @lc code=end

