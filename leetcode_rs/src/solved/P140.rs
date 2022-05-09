use super::Solution;
/*
 * @lc app=leetcode.cn id=140 lang=rust
 *
 * [140] 单词拆分 II
 */

// @lc code=start
use std::{collections::{HashSet, HashMap, VecDeque}};

impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> Vec<String> {
        let mut res = vec![];
        let mut table = HashSet::new();
        for i in word_dict{
            table.insert(i);
        }

        // dfs(0,&s,&table,&mut res,&mut vec![]);
        bfs(s, &table, &mut res);
        return res; 
    }
}

pub fn dfs(start:usize,s:&String,table:&HashSet<String>,res:&mut Vec<String>,str:&mut Vec<String>){
    if start == s.len(){
        let mut tmp = String::new();
        let target = str.iter().fold(&mut tmp, |target,x|{
            target.extend(x.chars());
            target.push(' ');
            target 
        });
        target.pop();
        res.push(target.clone());
    }
    for i in start+1..=s.len(){
        let prefix = s[start..i].to_string();
        if table.get(&prefix).is_some(){
            str.push(prefix);
            dfs(i, s, table, res, str);
            str.pop();
        }
    }
}

fn bfs(s:String,table:&HashSet<String>,res:&mut Vec<String>){
    let mut queue:VecDeque<(usize,Vec<String>)> = VecDeque::new();
    queue.push_back((0,vec![]));
    while !queue.is_empty(){
        let (start,mut str) = queue.pop_front().unwrap();
        for i in start+1..=s.len(){
            let prefix = s[start..i].to_string();
            if table.get(&prefix).is_some(){
                str.push(prefix);
                if i < s.len(){
                    queue.push_back((i,str.clone()));
                }else{
                    let mut tmp = String::new();
                    str.iter().fold(&mut tmp, |tmp,x|{
                        tmp.extend(x.chars());
                        tmp.push(' ');
                        tmp
                    });
                    tmp.pop();
                    res.push(tmp);
                }
                str.pop();
            }

        }
    }
}

// @lc code=end

#[test]
fn test(){
    let s = "catsanddog".to_string();
    let word_dict:Vec<String> = ["cat","cats","and","sand","dog"].iter().map(|x|x.to_string()).collect();
    let ans = ["cats and dog","cat sand dog"].iter().map(|x|x.to_string()).collect::<Vec<String>>();
    let res = Solution::word_break(s, word_dict);
    assert_eq!(res,ans);
}