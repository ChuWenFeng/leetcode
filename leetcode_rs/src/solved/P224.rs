use super::Solution;
/*
 * @lc app=leetcode.cn id=224 lang=rust
 *
 * [224] 基本计算器
 */

// @lc code=start
impl Solution {
    pub fn calculate(s: String) -> i32 {
        let sc:Vec<char> = s.chars().collect();
        // dfs(&sc,&mut 0)
        let mut num = 0;
        let mut res = 0;
        let mut sign = 1;
        let mut stack:Vec<(i32,i32)> = vec![];
        let mut right = 0;

        while right < sc.len(){
            match sc[right]{
                '('=>{
                    stack.push((res,sign));
                    res = 0;
                    sign = 1;
                },
                ')'=>{
                    res+=num*sign;
                    num = 0;
                    let top = stack.pop().unwrap();
                    res = res*top.1 + top.0;
                }
                n @ '0'..='9'=>{
                    num = num*10 + n as i32 - '0' as i32;

                },
                '+'=>{
                    res+=num*sign;
                    num = 0;
                    sign = 1;
                },
                '-'=>{
                    res+= num * sign;
                    num = 0;
                    sign = -1;
                }
                _=>{}
            }
            right+=1;
        }


        res+num*sign
    }
}

fn dfs(s:&Vec<char>,right:&mut usize)->i32{
    let mut num = 0;
    let mut res = 0;
    let mut sign = 1;
    while *right < s.len(){
        match s[*right]{
            '('=>{
                *right +=1;
                res+=dfs(s, right)*sign;
            },
            ')'=>{
                res+=num*sign;
                return res;
            }
            '+'=>{
                res+=sign*num;
                num = 0;
                sign = 1;
            },
            '-'=>{
                res+=sign*num;
                num = 0;
                sign = -1;
            },
            n @ '0'..='9'=>{
                num = num*10 + n as i32 - '0' as i32;
            },
            _=>{}
        }
        *right+=1;
    }

    res+num*sign
}
// @lc code=end

#[test]
fn test(){
    let s = "(1+(4+5+2)-3)+(6+8)".to_string();
    let ans = 23;
    let res = Solution::calculate(s);
    assert_eq!(res,ans);
}