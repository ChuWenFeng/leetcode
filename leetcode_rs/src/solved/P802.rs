use super::Solution;

/*
 * @lc app=leetcode.cn id=802 lang=rust
 *
 * [802] 找到最终的安全状态
 */

// @lc code=start
impl Solution {
    pub fn eventual_safe_nodes(graph: Vec<Vec<i32>>) -> Vec<i32> {
        let mut reverin = vec![0;graph.len()];
        let mut revergraph = vec![vec![];graph.len()];
        let mut ans = vec![];
        let mut queue = std::collections::VecDeque::new();
        for (idx,node) in graph.iter().enumerate(){
            reverin[idx] = node.len();
            for &j in node{
                revergraph[j as usize].push(idx);
            }
        }
        for (idx,&num) in reverin.iter().enumerate(){
            if num == 0{
                queue.push_back(idx);
            }
        }

        while let Some(node) = queue.pop_front() {
            for &i in revergraph[node].iter(){
                reverin[i]-=1;
                if reverin[i] == 0{
                    queue.push_back(i);
                }
            }
            ans.push(node as i32);
        }

        ans.sort();

        return ans;
    }
}
// @lc code=end

