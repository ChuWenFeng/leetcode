use super::Solution;
/*
 * @lc app=leetcode.cn id=1021 lang=rust
 *
 * [1021] 删除最外层的括号
 */

// @lc code=start
impl Solution {
    pub fn remove_outer_parentheses(s: String) -> String {
        let mut iter = s.chars();
        let mut flag = false;
        let mut prim = String::new();
        let mut count = 0;
        for c in iter{
            match c{
                '('=>{
                    if !flag{
                        flag = true;
                        continue;
                    }
                    count+=1;
                    prim.push(c);
                },
                ')'=>{
                    if count == 0{
                        flag = false;
                        continue;
                    }
                    count-=1;
                    prim.push(c);
                },
                _=>{
                    prim.push(c);
                }
            }
        }

        prim
    }
}
// @lc code=end

