
use super::Solution;
/*
 * @lc app=leetcode.cn id=1728 lang=rust
 *
 * [1728] 猫和老鼠 II
 */

// @lc code=start
use std::{collections::VecDeque};

 
const MouseTurn:usize = 0;
const CatTurn:usize   = 1;
const UNKNOWN:usize   = 0;
const MouseWin:usize  = 1;
const CatWin :usize   = 2;
const MaxMoves :usize = 1000;


impl Solution {//result err 140
    pub fn can_mouse_win(grid: Vec<String>, cat_jump: i32, mouse_jump: i32) -> bool {
        let grid:Vec<Vec<char>> = grid.iter().map(|row|row.chars().collect()).collect();
        let mut dirs = vec![(-1,0),(1,0),(0,-1),(0,1)];
        let rows = grid.len();
        let cols = grid[0].len();
        
        let mut startMouse = 0;
        let mut startCat  = 0;
        let mut food = 0;
        let getPos = |row:usize,col:usize| {
            return row*cols+col;
        };

        for (i,row) in grid.iter().enumerate(){
            for (j,&ch) in row.iter().enumerate(){
                if ch == 'M'{
                    startMouse = getPos(i, j);
                }
                if ch == 'C'{
                    startCat = getPos(i,j);
                }
                if ch == 'F'{
                    food = getPos(i, j);
                }
            }
        }

        let total = rows*cols;
        let mut degrees = vec![vec![vec![0;2];64];64];

        for mouse in 0..total{
            let mouseRow = mouse/cols;
            let mouseCol = mouse % cols;

            if grid[mouseRow][mouseCol] == '#'{
                continue;
            }

            for cat in 0..total{
                let catRow = cat/cols;
                let catCol = cat%cols;
                if grid[catRow][catCol] == '#'{
                    continue;
                }
                degrees[mouse][cat][MouseTurn]+=1;
                degrees[mouse][cat][CatTurn]+=1;

                for dir in dirs.iter(){
                    let mut row:i32 = mouseRow as i32+dir.0;
                    let mut col:i32 = mouseCol as i32+dir.1;
                    let mut jump = 1;
                    while row >= 0 && row <rows as i32 && col>=0 && col <cols as i32 && grid[row as usize][col as usize]!='#' && jump <= mouse_jump {
                        
                        let nextMouse = getPos(row as usize,col as usize);
                        let nextCat = getPos(catRow,catCol);
                        degrees[nextMouse][nextCat][MouseTurn]+=1;
                       
                        row+=dir.0;
                        col+=dir.1;

                        jump+=1;
                    }
                    row = catRow as i32 + dir.0;
                    col = catCol as i32 + dir.1;
                    jump = 1;
                    while row >= 0 && row <rows as i32 && col>=0 && col <cols as i32 && grid[row as usize][col as usize]!='#' && jump <= cat_jump {
                        let nextMouse = getPos(mouseRow, mouseCol);
                        let nextCat = getPos(row as usize,col as usize);
                        degrees[nextMouse][nextCat][MouseTurn]+=1;
                       
                        row+=dir.0;
                        col+=dir.1;

                        jump+=1;
                    }
                }
            }
        }

        let mut results = vec![vec![vec![vec![0;2];2];64];64];

        let mut q = VecDeque::new();

        for pos in 0..total{
            let row = pos/cols;
            let col = pos % cols;
            if grid[row][col] == '#'{
                continue;
            }
            results[pos][pos][MouseTurn][0] = CatWin;
            results[pos][pos][MouseTurn][1] = 0;
            results[pos][pos][CatTurn][0] = CatWin;
            results[pos][pos][CatTurn][1] = 0;
            q.push_back((pos,pos,MouseTurn));
            q.push_back((pos,pos,CatTurn));
        }

        for mouse in 0..total{
            let mouseRow = mouse / cols;
            let mouseCol = mouse % cols;
            if grid[mouseRow][mouseCol] == '#' || mouse == food {
                continue;
            }
            results[mouse][food][MouseTurn][0] = CatWin;
            results[mouse][food][MouseTurn][1] = 0;
            results[mouse][food][CatTurn][0] = CatWin;
            results[mouse][food][CatTurn][1] = 0;

            q.push_back((mouse,food,MouseTurn));
            q.push_back((mouse,food,CatTurn));

        }

        for cat in 0..total{
            let catRow = cat / cols;
            let catCol = cat % cols;
            if grid[catRow][catCol] == '#' || cat == food {
                continue;
            }
            results[food][cat][MouseTurn][0] = MouseWin;
            results[food][cat][MouseTurn][1] = 0;
            results[food][cat][CatTurn][0] = MouseWin;
            results[food][cat][CatTurn][1] = 0;
            q.push_back((food,cat,MouseTurn));
            q.push_back((food,cat,CatTurn));

        }

        let mut getPrevStatus = |mouse:usize,cat:usize,turn:usize|->Vec<(usize, usize, usize)>{
            let mouseRow = mouse / cols;
            let mouseCol = mouse % cols;
            let catRow = cat/cols;
            let catCol = cat%cols;

            let mut prevTurn = MouseTurn;
            if turn == MouseTurn{
                prevTurn = CatTurn
            }
            let (mut maxjump,mut startRow,mut startCol) = (cat_jump,catRow,catCol);
            if prevTurn == MouseTurn{
                maxjump = mouse_jump;
                startRow = mouseRow;
                startCol = mouseCol;
            }
            let mut prevStates = vec![];
            for dir in dirs.iter(){
                let mut i = startRow as i32 +dir.0;
                let mut j = startCol as i32 + dir.1;
                let mut jump = 1;
                while i>=0 && i< rows as i32 && j >= 0 && j < cols as i32 && grid[i as usize][j as usize] != '#' && jump <= maxjump{

                    let mut prevMouseRow = mouseRow;
                    let mut prevMouseCol = mouseCol;
                    let mut prevCatRow = i as usize;
                    let mut prevCatCol = j as usize;
                    if prevTurn == MouseTurn{
                        prevMouseRow = i as usize;
                        prevMouseCol = j as usize;
                        prevCatRow = catRow;
                        prevCatCol = catCol;
                    }
                    let prevMouse = getPos(prevMouseRow,prevMouseCol);
                    let prevCat = getPos(prevCatRow,prevCatCol);
                    prevStates.push((prevMouse,prevCat,prevTurn));
                    i += dir.0;
                    j += dir.1;
                    jump+=1;
                }
            }
            return prevStates;
        };

        while !q.is_empty(){
            let s = q.pop_front().unwrap();
            let (mouse,cat,turn) = s;
            let result = results[mouse][cat][turn][0];
            let moves = results[mouse][cat][turn][1];

            for (prevMouse,prevCat,prevTurn) in getPrevStatus(mouse,cat,turn){
                if results[prevMouse][prevCat][prevTurn][0] == UNKNOWN{
                    let canWin = (result == MouseWin && prevTurn == MouseTurn) || (result == CatWin && prevTurn == CatTurn);

                    if canWin{
                        results[prevMouse][prevCat][prevTurn][0] = result;
                        results[prevMouse][prevCat][prevTurn][1] = moves + 1;
                        q.push_back((prevMouse,prevCat,prevTurn));
                    }else{
                        degrees[prevMouse][prevCat][prevTurn]-=1;
                        if degrees[prevMouse][prevCat][prevTurn] == 0{
                            let mut loseResult = MouseWin;
                            if prevTurn == MouseTurn{
                                loseResult = CatWin;
                            }
                            results[prevMouse][prevCat][prevTurn][0] = loseResult;
                            results[prevMouse][prevCat][prevTurn][1] = moves + 1;
                            q.push_back((prevMouse,prevCat,prevTurn));

                        }
                    }

                }
            }
        }

        return results[startMouse][startCat][MouseTurn][0] == MouseWin && results[startMouse][startCat][MouseTurn][1] <= MaxMoves;

    }
}


// @lc code=end

