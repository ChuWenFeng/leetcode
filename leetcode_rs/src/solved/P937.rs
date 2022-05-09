use super::Solution;
/*
 * @lc app=leetcode.cn id=937 lang=rust
 *
 * [937] 重新排列日志文件
 */

// @lc code=start
use std::{cmp::Ordering};
impl Solution {
    pub fn reorder_log_files(logs: Vec<String>) -> Vec<String> {
        let mut diglist = vec![];
        let mut letlist = vec![];
        for i in logs.iter(){
            let mut it = i.split(' ');
            it.next();
            let firstc = it.next().unwrap();
            let mut digflag = true;
            for c in firstc.chars(){
                match c {
                    c if c>='0'&&c<='9'=>{
                        continue;
                    },
                    _=>{
                        digflag = false;
                        break;
                    }
                }
            }
            if digflag{
                diglist.push(i.clone());
            }else{
                letlist.push(i.clone());
            }
        }
        let sort = |a:&String,b:&String|->Ordering{
            let mut aiter = a.split(' ');
            let mut biter = b.split(' ');
            let taga = aiter.next().unwrap();
            let tagb = biter.next().unwrap();
            let mut ac = aiter.next();
            let mut bc = biter.next();
            while ac.is_some() && bc.is_some(){
                let acc = ac.unwrap();
                let bcc = bc.unwrap();
                ac = aiter.next();
                bc = biter.next();
                match acc.cmp(&bcc){
                    Ordering::Equal=>{
                        continue;
                    },
                    ord =>{
                        return ord;
                    }
                }
            }
            if ac.is_some(){
                return Ordering::Greater;
            }else if bc.is_some(){
                return Ordering::Less;
            }else{
                return taga.cmp(&tagb);
            }
        };
        letlist.sort_by(sort);

        letlist.append(&mut diglist);

        letlist
    }
}
// @lc code=end

#[test]
fn test(){
    let logs:Vec<String> = vec!["a1 9 2 3 1","g1 act car","zo4 4 7","ab1 off key dog","a8 act zoo","a2 act car"].iter().map(|log|log.to_string()).collect();
    let ans:Vec<String> = vec!["a2 act car","g1 act car","a8 act zoo","ab1 off key dog","a1 9 2 3 1","zo4 4 7"].iter().map(|log|log.to_string()).collect();
    let res = Solution::reorder_log_files(logs);
    assert_eq!(res,ans);
}
#[test]
fn test2(){
    let logs:Vec<String> = vec!["let1 art can","let3 art zero"].iter().map(|log|log.to_string()).collect();
    let ans:Vec<String> = vec!["let1 art can","let3 art zero"].iter().map(|log|log.to_string()).collect();
    let res = Solution::reorder_log_files(logs);
    assert_eq!(res,ans);
}