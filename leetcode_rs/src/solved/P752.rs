use super::Solution;

/*
 * @lc app=leetcode.cn id=752 lang=rust
 *
 * [752] 打开转盘锁
 */

// @lc code=start
use std::collections::{VecDeque, HashMap, HashSet};
impl Solution {
    pub fn open_lock(deadends: Vec<String>, target: String) -> i32 {
        let mut deadtab = HashSet::new();
        for i in deadends{
            let mut clist = [0;4];
            for (idx,c) in i.chars().enumerate(){
                clist[idx] = c as u8;
            }
            deadtab.insert(clist);
        }
        let mut start = ['0' as u8;4];
        if deadtab.contains(&start){
            return -1;
        }
        let mut targ = [0;4];
        for (idx,c) in target.chars().enumerate(){
            targ[idx] = c as u8;
        }
        if start == targ{
            return 0;
        }
        let mut seen:HashSet<[u8;4]> = HashSet::new();

        let mut queue:VecDeque<([u8;4],i32)> = VecDeque::new();
        queue.push_back((start,0));
        let bfs = |mut status:[u8;4]|->Vec<[u8;4]>{
            let mut ret = vec![];
            let mut s = status;
            for (i,&b) in status.iter().enumerate(){
                s[i] = b - 1;
                if s[i] < '0' as u8{
                    s[i] = '9' as u8;
                }
                ret.push(s);

                s[i] = b + 1;
                if s[i] > '9' as u8{
                    s[i] = '0' as u8;
                }
                ret.push(s);
                s[i] = b;
            }
            ret
        };

        while !queue.is_empty(){
            let (status,step) = queue.pop_front().unwrap();
            for next_status in bfs(status){
                if !seen.contains(&next_status) && !deadtab.contains(&next_status){
                    if next_status == targ{
                        return step + 1;
                    }
                    seen.insert(next_status);
                    queue.push_back((next_status,step+1));
                }
            }
        }

        return -1;
    }
}
// @lc code=end

