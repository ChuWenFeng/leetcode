use super::Solution;
/*
 * @lc app=leetcode.cn id=329 lang=rust
 *
 * [329] 矩阵中的最长递增路径
 */

// @lc code=start
const dirs:[(i32,i32);4] = [(1,0),(-1,0),(0,1),(0,-1)];
impl Solution {
    pub fn longest_increasing_path(matrix: Vec<Vec<i32>>) -> i32 {
        let row = matrix.len();
        let col = matrix[0].len();
        let mut mem  = vec![vec![0;col];row];
        let mut ans = 0;
        for x in 0..row{
            for y in 0..col{
                ans = ans.max(dfs(&mut mem,&matrix,x as i32,y as i32));
            }
        }
        ans
    }
}

fn dfs(mem:&mut Vec<Vec<i32>>,matrix:&Vec<Vec<i32>>,x:i32,y:i32)->i32{
    if mem[x as usize][y as usize] != 0{
        return mem[x as usize][y as usize];
    }
    let row = matrix.len() as i32;
    let col = matrix[0].len() as i32;
    mem[x as usize][y as usize] = 1;

    for dir in dirs{
        let nx = x+dir.0;
        let ny = y+dir.1;
        if nx>=0 && nx<row && ny >= 0 && ny < col && matrix[x as usize][y as usize]<matrix[nx as usize][ny as usize]{
            mem[x as usize][y as usize] = mem[x as usize][y as usize].max(dfs(mem, matrix, nx, ny)+1);
        }
    }
    return mem[x as usize][y as usize];
}
// @lc code=end

