use super::Solution;
/*
 * @lc app=leetcode.cn id=417 lang=rust
 *
 * [417] 太平洋大西洋水流问题
 */

// @lc code=start
static mut row:usize = 0;
static mut col:usize = 0;

impl Solution {
    pub fn pacific_atlantic(heights: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        unsafe{
            row = heights.len();
            col = heights[0].len();
        }
        
        let mut ans = vec![];

        
        unsafe{
            let mut pacific = vec![vec![false;col];row];
            let mut atlantic = vec![vec![false;col];row];
            for i in 0..row{
                dfs(i, 0, &mut pacific, &heights);
            }
            for j in 1..col{
                dfs(0, j, &mut pacific, &heights);
            }
            for i in 0..row{
                dfs(i, col-1, &mut atlantic, &heights)
            }
            for j in 0..col-1{
                dfs(row-1,j,&mut atlantic,&heights);
            }
            for (i,(p,a)) in pacific.iter().zip(atlantic.iter()).enumerate(){
                for (j,(&left,&right)) in p.iter().zip(a.iter()).enumerate(){
                    if left && right{
                        ans.push(vec![i as i32,j as i32]);
                    }
                }
            }
        }

        ans
    }
}
fn dfs(x:usize,y:usize,ocean :&mut Vec<Vec<bool>>,heights:&Vec<Vec<i32>>){
    let dirs = [(-1,0),(1,0),(0,-1),(0,1)];
    if ocean [x][y]{
        return ;
    }
    ocean[x][y] = true;
    
    for (dx,dy) in dirs{
        let nx = x as i32 + dx;
        let ny = y as i32 + dy;
        unsafe{if 0<=nx && nx < row as i32 && 0 <= ny && ny < col as i32 && heights[nx as usize][ny as usize] >= heights[x][y]{
            dfs(nx as usize,ny as usize,ocean,heights);
        }}
    }
}
// @lc code=end

