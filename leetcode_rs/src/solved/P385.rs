use super::Solution;

#[derive(Debug, PartialEq, Eq)]
pub enum NestedInteger {
  Int(i32),
  List(Vec<NestedInteger>)
}
/*
 * @lc app=leetcode.cn id=385 lang=rust
 *
 * [385] 迷你语法分析器
 */

// @lc code=start
// #[derive(Debug, PartialEq, Eq)]
// pub enum NestedInteger {
//   Int(i32),
//   List(Vec<NestedInteger>)
// }
impl Solution {
    pub fn deserialize(s: String) -> NestedInteger {
        let mut sc:Vec<char> = s.chars().collect();
        if sc[0] != '['{
            let num = s.parse::<i32>().unwrap();
            return NestedInteger::Int(num);
        }
        
        let mut stack = std::collections::VecDeque::new();
        let mut curr_num = 0;
        let mut bit_flag = 1;
        let mut num_flag = false;
        for c in sc{
            match c{
                '['=>{
                    stack.push_back(NestedInteger::List(vec![]));
                },
                ']'=>{
                    let mut top = stack.pop_back().unwrap();

                    if num_flag{
                        let ni = NestedInteger::Int(curr_num*bit_flag);
                        if let NestedInteger::List(v) = &mut top{
                            v.push(ni);
                        }
                        num_flag = false;
                    }
                    
                    match stack.back_mut(){
                        Some(NestedInteger::List(vec))=>{
                            vec.push(top)
                        },
                        _=>{
                            return top;
                        },
                    }
                    
                    curr_num = 0;
                    bit_flag = 1;
                },
                ','=>{
                    if num_flag{
                        let ni = NestedInteger::Int(curr_num*bit_flag);
                        match stack.back_mut(){
                            Some(NestedInteger::List(vec))=>{
                                vec.push(ni);
                            },
                            _=>{}
                        }
                        num_flag = false;
                    }
                    
                    curr_num = 0;
                    bit_flag = 1;
                },
                '0'..='9' =>{
                    num_flag = true;
                    curr_num = curr_num*10 + (c as i32 - '0' as i32);
                },
                '-'=>{
                    bit_flag = -1;
                }
                _=>{}
            }

        }

        return stack.pop_back().unwrap();
    }
}
// @lc code=end

#[test]
fn test(){
    let mut input = "[123,456,[788,799,833],[[]],10,[]]"
    .to_string();
    let res = Solution::deserialize(input);
    println!("{:?}",res);
}