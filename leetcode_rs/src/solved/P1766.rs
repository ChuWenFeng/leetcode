use super::Solution;

/*
 * @lc app=leetcode.cn id=1766 lang=rust
 *
 * [1766] 互质树
 */

// @lc code=start
impl Solution {
    pub fn get_coprimes(nums: Vec<i32>, edges: Vec<Vec<i32>>) -> Vec<i32> {
        let mut len = nums.len();
        let mut grap:Vec<Vec<i32>> = vec![vec![];len];
        for e in edges{
            grap[e[0] as usize].push(e[1]);
            grap[e[1] as usize].push(e[0]);
        }
        let mut res = vec![-1;len];
        let mut lasts = vec![vec![];55];
        dfs(&grap,&mut res,&mut lasts,0,-1,0,&nums);


        res
    }
}

fn dfs(grap:&Vec<Vec<i32>>,res:&mut Vec<i32>,lasts:&mut Vec<Vec<(i32,i32)>>,node:i32,pre:i32,level:i32,nums:&Vec<i32>){

    let mut re = -1;
    let mut lev = -1;
    for i in 1..=50{
        if lasts[i].len()>0 && lasts[i].last().unwrap().0 > lev && gcd(i as i32,nums[node as usize]) == 1{
            re = lasts[i].last().unwrap().1;
            lev = lasts[i].last().unwrap().0;
        }
    }
    res[node as usize] = re;

    for &ne in grap[node as usize].iter(){
        if ne != pre{
            lasts[nums[node as usize]as usize].push((level,node));
            dfs(grap,res,lasts,ne,node,level+1,nums);
            lasts[nums[node as usize] as usize].pop();
        }
    }

}

fn gcd(x:i32,y:i32)->i32{
    if x%y == 0{
        return y;
    }else{
        return gcd(y,x%y);
    }
}

// @lc code=end

