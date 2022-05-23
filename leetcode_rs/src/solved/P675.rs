use super::Solution;
/*
 * @lc app=leetcode.cn id=675 lang=rust
 *
 * [675] 为高尔夫比赛砍树
 */

// @lc code=start
use std::collections::VecDeque;
impl Solution {
    pub fn cut_off_tree(forest: Vec<Vec<i32>>) -> i32 {
        let mut ans = 0;

        let mut list = vec![];
        for (rown,row) in forest.iter().enumerate(){
            for (coln,&v) in row.iter().enumerate(){
                if v > 1{
                    list.push((v,rown,coln));
                }
            }
        }
        list.sort_by(|a,b|{
            a.0.cmp(&b.0)
        });
        let (mut curx,mut cury) = (0,0);
        for next in list{
            let v = bfs(curx, cury, next.1 as i32, next.2 as i32, &forest);
            if v==-1{
                return -1;
            }
            ans+=v;
            curx = next.1 as i32;
            cury = next.2 as i32;
        }

        ans
    }
}


fn bfs(px:i32,py:i32,tx:i32,ty:i32,forest:&Vec<Vec<i32>>)->i32{
    let row = forest.len();
    let col = forest[0].len();
    let mut vis = vec![vec![false;col];row];
    vis[px as usize][py as usize] = true;
    let mut queue = VecDeque::new();
    let mut dir = vec![(0,1),(0,-1),(1,0),(-1,0)];
    queue.push_back((0,px,py));
    while !queue.is_empty(){
        let next = queue.pop_front().unwrap();
        if next.1 == tx && next.2 == ty{
            return next.0;
        }
        for &d in dir.iter(){
            let x = next.1+d.0;
            let y = next.2+d.1;
            if x>=0 && x<row as i32 && y>=0 && y<col as i32 && !vis[x as usize][y as usize] && forest[x as usize][y as usize] > 0{
                vis[x as usize][y as usize] = true;
                queue.push_back((next.0+1,x,y));
            }
        }
    }

    return -1;
}

// @lc code=end