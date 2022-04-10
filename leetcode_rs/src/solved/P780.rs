

use super::Solution;
/*
 * @lc app=leetcode.cn id=780 lang=rust
 *
 * [780] 到达终点
 */

// @lc code=start
impl Solution {
    pub fn reaching_points(sx: i32, sy: i32, tx: i32, ty: i32) -> bool {
        use std::cmp::Ordering;
        let (mut sx,mut sy,mut tx,mut ty) = (sx,sy,tx,ty);
        if tx == sx && ty == sy{
            return true;
        }
        if tx == ty || tx < sx || ty < sy{
            return false;
        }

        while tx > sx && ty > sy{
            match tx.cmp(&ty) {
                Ordering::Greater => {
                    tx = tx % ty;
                },
                Ordering::Less =>{
                    ty = ty % tx;
                },
                _ => return false
            }
        }

        if tx == sx && ty > sy && (ty-sy) % tx == 0{
            return true;
        }

        if ty == sy && tx > sx && (tx-sx) % ty == 0{
            return true;
        }
        return false;
    }
}
// @lc code=end

