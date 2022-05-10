
use super::Solution;
/*
 * @lc app=leetcode.cn id=913 lang=rust
 *
 * [913] 猫和老鼠
 */

// @lc code=start
use std::collections::VecDeque;
impl Solution {
    pub fn cat_mouse_game_dfs(graph: Vec<Vec<i32>>) -> i32 {
        let len = graph.len();
        let mut f = vec![vec![vec![-1;len];len];2*len*len];

        return dfs(0,1,2,1,2,&mut f,&graph);
    }
    pub fn cat_mouse_game(graph: Vec<Vec<i32>>) -> i32 {
        let len = graph.len();
        let mut degrees = vec![vec![vec![0;2];len];len];
        let mut results = vec![vec![vec![0;2];len];len];

        for (i,to) in graph.iter().enumerate(){
            for j in 1..len{
                degrees[i][j][0] = to.len();
                degrees[i][j][1] = graph[j].len();
            }
        }

        for &y in graph[0].iter(){
            for i in 0..len{
                degrees[i][y as usize][1]-=1;
            }
        }

        let mut q = VecDeque::new();

        for j in 1..len{
            results[0][j][0] = 1;
            results[0][j][1] = 1;
            q.push_back((0,j,0));
            q.push_back((0,j,1));
        }

        for i in 1..len{
            results[i][i][0] = 2;
            results[i][i][1] = 2;
            q.push_back((i,i,0));
            q.push_back((i,i,1));
        }

        while !q.is_empty(){
            let s = q.pop_front().unwrap();
            let result = results[s.0][s.1][s.2];
            for p in getPrevState(s, &graph){
                let (prevMouse,prevCat,prevTurn) = p;
                if results[prevMouse][prevCat][prevTurn] == 0{
                    let canWin = (result == 1 && prevTurn == 0) || (result == 2 && prevTurn == 1);
                    if canWin{
                        results[prevMouse][prevCat][prevTurn] = result;
                        q.push_back(p);
                    }else{
                        degrees[prevMouse][prevCat][prevTurn]-=1;
                        if degrees[prevMouse][prevCat][prevTurn] == 0{
                            if prevTurn == 0{
                                results[prevMouse][prevCat][prevTurn] = 2;
                            }else{
                                results[prevMouse][prevCat][prevTurn] = 1;
                            }
                            q.push_back(p);
                        }
                    }
                }
            }

        }

        return results[1][2][0];
    }
}

fn getPrevState(s:(usize,usize,usize),graph:&Vec<Vec<i32>>)->Vec<(usize,usize,usize)>{
    let mut prevStates = vec![];
    if s.2 == 0{
        for &prev in graph[s.1].iter(){
            if prev != 0{
                prevStates.push((s.0,prev as usize,1));
            }
        }
    }else{
        for &prev in graph[s.0].iter(){
            prevStates.push((prev as usize,s.1,0));
        }
    }

    prevStates
}

fn dfs(k:usize,a:usize,b:usize,prea:usize,preb:usize,f:&mut Vec<Vec<Vec<i32>>>,graph:&Vec<Vec<i32>>)->i32{
    if k >= f.len(){
        return 0;
    }
    let mut ans = f[k][a][b];
    if a == 0{
        ans = 1;
    }else if a==b{
        ans = 2;
    }else if k >= 2 * graph.len() * graph.len(){
        ans = 0;
    }else if ans == -1{
        if k & 1 == 0{
            let mut win = false;
            let mut draw = false;
            for &ne in graph[a].iter(){
                if ne as usize == prea{
                    continue;
                }
                let t = dfs(k+1, ne as usize, b,a,b, f, graph);
                if t == 1{
                    win = true;
                }else if t == 0 {
                    draw = true;
                }
                if win{
                    break;
                }
            }
            if win{
                ans = 1;
            }else if draw{
                ans = 0;
            }else{
                ans = 2;
            }
        }else{
            let mut win = false;
            let mut draw = false;
            for &ne in graph[b].iter(){
                if ne == 0 || ne as usize == preb{
                    continue;
                }
                let t = dfs(k+1, a, ne as usize, a,b,f, graph);
                if t == 2{
                    win = true;
                }else if t == 0 {
                    draw = true;
                }
                if win{
                    break;
                }
            }
            if win{
                ans = 2;
            }else if draw {
               ans = 0; 
            }else{
                ans = 1;
            }
        }

    }

    f[k][a][b] = ans;
    ans
}
// @lc code=end


