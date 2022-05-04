use super::Solution;
/*
 * @lc app=leetcode.cn id=20 lang=rust
 *
 * [20] 有效的括号
 */

// @lc code=start
impl Solution {
    pub fn is_valid_20(s: String) -> bool {
        
        let mut stack = vec![];
        for i in s.chars(){
            match i{
                '{'|'['|'('=>{
                    stack.push(i);
                },
                '}'=>{
                    if let Some(c) = stack.pop(){
                        if c != '{'{
                            return false;
                        }
                    }else{
                        return false;
                    }
                },
                ']'=>{
                    if let Some(c) = stack.pop(){
                        if c != '['{
                            return false;
                        }
                    }else{
                        return false;
                    }
                },
                ')'=>{
                    if let Some(c) = stack.pop(){
                        if c != '('{
                            return false;
                        }
                    }else{
                        return false;
                    }
                },
                _=>{return false;},
            }
        }
        if stack.len() != 0{
            return false;
        }
        
        return true;
    }
}
// @lc code=end

#[test]
fn test(){
    let input = "()[]{}".to_string();
    let ans = true;
    let res = Solution::is_valid_20(input);
    assert_eq!(res,ans);
}