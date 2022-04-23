use super::Solution;
/*
 * @lc app=leetcode.cn id=200 lang=rust
 *
 * [200] 岛屿数量
 */

// @lc code=start
impl Solution {
    //深度优先搜索
    pub fn num_islands_dfs(grid: Vec<Vec<char>>) -> i32 {
        let mut grid = grid.clone();
        let mut row = grid.len();
        let col = grid[0].len();
        let mut ans = 0;
        for i in 0..row{
            for j in 0..col{
                if grid[i][j] == '1'{
                    dfs(&mut grid, i, j, row, col);
                    ans+=1;
                }
            }
        }
        return ans;
    }

    //并查集
    pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
        let mut ans = 0;
        let row = grid.len();
        let col = grid[0].len();
        let mut parent = vec![0;row*col];

        for i in 0..row{
            for j in 0..col{
                let idx = i*col+j;
                parent[idx] = idx;
                if grid[i][j] == '1'{
                    ans+=1;
                }
            }
        }

        for i in 0..row{
            for j in 0..col{
                let idx = i*col+j;
                if grid[i][j] == '1'{
                    if i+1<row && grid[i+1][j] == '1'{
                        union(&mut parent,idx,idx+col,&mut ans);
                    }
                    if j+1 < col && grid[i][j+1] == '1'{
                        union(&mut parent,idx,idx+1,&mut ans);
                    }
                }

            }
        }

        ans
    }
}


fn find(parent:&mut Vec<usize>,idx:usize)->usize{
    if parent[idx] != idx{
        parent[idx] = find(parent,parent[idx]);
    }
    return parent[idx]
}

fn union(parent:&mut Vec<usize>,i:usize,j:usize,ans:&mut i32){
    let rooti = find(parent,i);
    let rootj = find(parent, j);
    if rooti == rootj {return;}

    parent[rooti] = parent[rootj];
   
    *ans-=1;
}

fn dfs(grid:&mut Vec<Vec<char>>,row:usize,col:usize,rline:usize,cline:usize){
    if grid[row][col] == '1'{
        grid[row][col] = '0';
        if row>=1 && grid[row-1][col] == '1' {dfs(grid, row-1, col, rline, cline)}
        if row+1 < rline && grid[row+1][col] == '1' {dfs(grid, row+1, col, rline, cline)}
        if col>=1 && grid[row][col-1] == '1' {dfs(grid, row, col-1, rline, cline)}
        if col+1<cline && grid[row][col+1] == '1'{dfs(grid, row, col+1, rline, cline)}
    }else{
        return;
    }
}
// @lc code=end

#[test]
fn test(){
    let grid = vec![
            vec!['1','0','1','1','1'],
            vec!['1','0','1','0','1'],
            vec!['1','1','1','0','1']
        ];
    let ans = 1;
    
    let res = Solution::num_islands(grid);
    assert_eq!(res,ans);
      
}