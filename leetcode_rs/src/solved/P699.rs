use super::Solution;
/*
 * @lc app=leetcode.cn id=699 lang=rust
 *
 * [699] 掉落的方块
 */

// @lc code=start
use std::collections::BTreeMap;
impl Solution {
    pub fn falling_squares(positions: Vec<Vec<i32>>) -> Vec<i32> {
        let n = positions.len();
        let mut heighs = vec![0;n];

        for (i,p) in positions.iter().enumerate(){
            let size = p[1];
            let left = p[0];
            let right = left+size-1;
            heighs[i] = size;
            for (j,q) in positions[..i].iter().enumerate(){
                let froleft = q[0];
                let forright = q[0]+q[1]-1;
                if right>=froleft && left <= forright{
                    heighs[i] = heighs[i].max(heighs[j]+size);
                }
            }
        }
        for i in 1..n{
            heighs[i] = heighs[i-1].max(heighs[i]);
        }

        heighs
    }
}
// @lc code=end

