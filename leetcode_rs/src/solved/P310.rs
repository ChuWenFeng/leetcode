use super::Solution;

/*
* @lc app=leetcode.cn id=310 lang=rust
*
* [310] 最小高度树
*/

// @lc code=start
impl Solution {
   pub fn find_min_height_trees(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        if n == 1{
            return vec![0];
        }
        let mut n = n as usize;
        let mut adjlist = vec![vec![];n]; 
        let mut degree = vec![0;n];
        for i in edges{
            let x = i[0] as usize;
            let y = i[1] as usize;
            adjlist[x].push(y);
            degree[x]+=1;
            adjlist[y].push(x);
            degree[y]+=1;
        }

        let mut q = vec![];
        for (node,&deg) in degree.iter().enumerate(){
            if deg == 1{
                q.push(node);
            }
        }
        let mut tmp;
        while n>2{
            n-=q.len();
            tmp = q.clone();
            q.clear();
            for &i in tmp.iter(){
                for &j in adjlist[i].iter(){
                    degree[j] -=1;
                    if degree[j] == 1{
                        q.push(j);
                    }
                }
            }
        }
        return q.iter().map(|&x|x as i32).collect();
   }
}
// @lc code=end


