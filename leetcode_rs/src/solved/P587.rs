use super::Solution;

/*
 * @lc app=leetcode.cn id=587 lang=rust
 *
 * [587] 安装栅栏
 */

// @lc code=start
impl Solution {
    pub fn outer_trees(trees: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut trees = trees.clone();
        let mut ans = vec![];
        trees.sort_by(|a,b|{
            if a[0]==b[0]{
                return a[1].cmp(&b[1]);
            }else{
                return a[0].cmp(&b[0]);
            }
        });

        let len = trees.len();
        let mut tp = 1;
        let mut stack = vec![0;len+10];
        stack[tp] = 0;
        let mut vis = vec![false;len+10];
        for i in 1..len{
            let c = &trees[i];
            while tp >= 2{
                let a = &trees[stack[tp-1]];
                let b = &trees[stack[tp]];
                if getArea(a, b, c) < 0.0   {
                    vis[stack[tp]] = false;
                    tp-=1;
                }else{
                    break;
                }
            }
            tp+=1;
            stack[tp] = i;
            vis[i] = true;
        }
        let size = tp;
        for i in (0..len).rev(){
            if vis[i] {continue};
            let c = &trees[i];
            while tp>size{
                let a = &trees[stack[tp-1]];
                let b = &trees[stack[tp]];
                if getArea(a, b, c) < 0.0{
                    tp-=1;
                }else{
                    break;
                }
            }
            tp+=1;
            stack[tp] = i;
        }

        for i in 1..tp{
            ans.push(trees[stack[i]].clone());
        }

        ans
    }
}

fn subtraction(a:&Vec<i32>,b:&Vec<i32>)->Vec<i32>{
    return vec![a[0]-b[0],a[1]-b[1]];
}

fn cross(a:&[i32],b:&[i32]) ->f32 {
    return (a[0] as f32 * b[1] as f32) - ( a[1] as f32 * b[0] as f32);
}

fn getArea(a:&Vec<i32>,b:&Vec<i32>,c:&Vec<i32>)->f32{
    return cross(&subtraction(b,a), &subtraction(c, a));
}
// @lc code=end

