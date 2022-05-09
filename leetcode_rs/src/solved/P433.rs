

use super::Solution;
/*
 * @lc app=leetcode.cn id=433 lang=rust
 *
 * [433] 最小基因变化
 */

// @lc code=start
use std::{collections::{HashSet, VecDeque, HashMap}, string};
impl Solution {
    pub fn min_mutation(start: String, end: String, bank: Vec<String>) -> i32 {
        let mut endindex = usize::MAX;
        bank.iter().enumerate().fold(&mut endindex, | endidx,(idx,x)|{
            if *x == end{
                *endidx = idx;
            }
            endidx
        });
        if endindex == usize::MAX{
            return -1;
        }
        let mut adj:Vec<Vec<usize>> = vec![vec![];bank.len()];
        for i in 0..bank.len()-1{
            for j in i+1..bank.len(){
                if diffOne(&bank[i], &bank[j]){
                    adj[i].push(j);
                    adj[j].push(i);
                }
            }
        }

        let mut queue = vec![];
        let mut vis = HashSet::new();

        for (idx,s) in bank.iter().enumerate(){
            if diffOne(&start, s){
                queue.push(idx);
                vis.insert(idx);
            }
        }

        let mut step = 1;
        while !queue.is_empty(){
            let tmp = queue.clone();
            queue = vec![];

            for idx in tmp{
                if idx == endindex{
                    return step;
                }
                for &nxt in adj[idx].iter(){
                    if !vis.contains(&nxt){
                        vis.insert(nxt);
                        queue.push(nxt);
                    }
                }

            }
            step+=1;
        }

        -1
    }
}

fn diffOne(s1:&String,s2:&String)->bool{
    let mut diff = false;
    for (c1,c2) in s1.chars().zip(s2.chars()){
        if c1 != c2{
            if diff{
                return false;
            }
            diff = true;
        }
    }
    return diff;
}
// @lc code=end

#[test]
fn test(){
    let start = "AACCGGTT".to_string();
    let end = "AAACGGTA".to_string();
    let bank:Vec<String> = ["AACCGATT","AACCGATA","AAACGATA","AAACGGTA"].iter().map(|x|x.to_string()).collect();
    let ans = 4;
    let res = Solution::min_mutation(start, end, bank);
    assert_eq!(res,ans);
}