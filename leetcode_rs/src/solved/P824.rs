use std::ops::Index;

use super::Solution;

/*
 * @lc app=leetcode.cn id=824 lang=rust
 *
 * [824] 山羊拉丁文
 */

// @lc code=start
impl Solution {
    pub fn to_goat_latin(sentence: String) -> String {
        let s = sentence.split(" ");
        let mut idx = 1;
        let mut ans = String::new();
        for i in s{
            let mut clist = String::new();
            let mut iterstr = i.chars();
            let mut cons = ' ';
            // let mut consflag = false;
            match iterstr.next(){
                Some(c)=>{
                    match c{
                        'a'|'e'|'i'|'o'|'u'|'A'|'E'|'I'|'O'|'U'=>{
                            clist.push(c);
                        },
                        _=>{
                            cons = c;
                            // consflag = true;
                        }
                    }
                },
                None=>{
                    continue;
                }
            }
            clist.extend(iterstr);
            if cons != ' '{
                clist.push(cons);
            }
            clist.push('m');
            clist.push('a');

            for _ in 0..idx{
                clist.push('a');
            }
            idx+=1;
            clist.push(' ');

            ans.extend(clist.chars());
        }
        ans.pop();
        ans
    }
}
// @lc code=end

