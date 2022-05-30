

use super::Solution;
/*
 * @lc app=leetcode.cn id=332 lang=rust
 *
 * [332] 重新安排行程
 */

// @lc code=start
use std::collections::HashMap;
impl Solution {
    pub fn find_itinerary(tickets: Vec<Vec<String>>) -> Vec<String> {
        let mut table:HashMap<String, Vec<String>> = HashMap::new();
        let mut res = vec![];

        for ticket in tickets.into_iter(){
            if let Some(v) = table.get_mut(&ticket[0]){
                v.push(ticket[1].clone());
            }else{
                table.insert(ticket[0].clone(), vec![ticket[1].clone()]);
            }
        }
        for (_,list) in table.iter_mut(){
            list.sort_by(|a,b|{
                b.cmp(a)
            });
        }
        dfs("JFK".to_string(), &mut table, &mut res);
        let ans:Vec<String> = res.into_iter().rev().collect();

        return ans;
    }
}

fn dfs(curr:String,table:&mut HashMap<String,Vec<String>>,ans:&mut Vec<String>){
        loop {
            if let Some(list) = table.get_mut(&curr){
                if list.is_empty(){
                    break;
                }
                let tmp = list.pop().unwrap();
                dfs(tmp, table,ans);

            }else{
                break;
            }
            
        }
        ans.push(curr.clone());
}
// @lc code=end

#[test]
fn test(){
    let tickets:Vec<Vec<String>> = [["MUC","LHR"],["JFK","MUC"],["SFO","SJC"],["LHR","SFO"]]
    .into_iter().map(|x|{
        vec![x[0].to_string(),x[1].to_string()]
    }).collect();
    let ans:Vec<String> = ["JFK","MUC","LHR","SFO","SJC"].iter().map(|x|x.to_string()).collect();
    let res = Solution::find_itinerary(tickets);
    assert_eq!(res,ans);
}