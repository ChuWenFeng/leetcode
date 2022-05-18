use super::Solution;
/*
 * @lc app=leetcode.cn id=289 lang=rust
 *
 * [289] 生命游戏
 */

// @lc code=start
impl Solution {
    pub fn game_of_life(board: &mut Vec<Vec<i32>>) {
        let preboard = board.clone();
        let row = board.len();
        let col = board[0].len();

        for x in 0..row{
            for y in 0..col{
                board[x][y] = match get_neighbor(&preboard, x, y){
                    1 => 0,
                    2 =>board[x][y],
                    3 => 1,
                    _=>0,
                }
            }
        }

    }
}

fn get_neighbor(board:&Vec<Vec<i32>>,x:usize,y:usize)->i32{
    let row = board.len();
    let col = board[0].len();
    let mut count = 0;

    if x >= 1{
        count += board[x-1][y];
    }
    if y >= 1{
        count += board[x][y-1];
    }

    if x < row-1{
        count+=board[x+1][y];
    }
    if y < col-1{
        count+=board[x][y+1];
    }
    if x >= 1 && y >= 1{
        count+=board[x-1][y-1];
    }
    if  x>=1 && y<col-1{
        count+=board[x-1][y+1];
    }
    if x<row-1 && y>=1{
        count+=board[x+1][y-1];
    }
    if x<row-1 && y< col-1{
        count+=board[x+1][y+1];
    }
    count
}
// @lc code=end

#[test]
fn test(){
    let mut board:Vec<Vec<i32>> = [[0,1,0],[0,0,1],[1,1,1],[0,0,0]].iter().map(|x|{
        let mut v = vec![];
        for &n in x{
            v.push(n);
        }
        v
    }).collect();
    let ans:Vec<Vec<i32>> = [[0,0,0],[1,0,1],[0,1,1],[0,1,0]].iter().map(|x|{
        let mut v = vec![];
        for &n in x{
            v.push(n);
        }
        v
    }).collect();
    Solution::game_of_life(&mut board);
    assert_eq!(board,ans);
}