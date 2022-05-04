use super::Solution;
/*
 * @lc app=leetcode.cn id=1074 lang=rust
 *
 * [1074] 元素和为目标值的子矩阵数量
 */

// @lc code=start
use std::collections::HashMap;
impl Solution {
    pub fn num_submatrix_sum_target(matrix: Vec<Vec<i32>>, target: i32) -> i32 {
        let row = matrix.len();
        let col = matrix[0].len();
        let mut ans = 0 ;

        let mut subarraySum = |nums:&Vec<i32>|{
            let mut table:HashMap<i32, i32> = HashMap::new();
            table.insert(0, 1);
            let mut pre = 0;
            for i in 0..nums.len(){
                pre+=nums[i];
                if let Some(&v) = table.get(&(pre-target)){
                    ans+=v;
                }
                if let Some(c) = table.get_mut(&pre){
                    *c+=1;
                }else{
                    table.insert(pre, 1);
                }
            }
        };

        for i in 0..row{
            let mut sum = vec![0;col];
            for row in i..row{
                for (idx,&v) in matrix[row].iter().enumerate(){
                    sum[idx]+=v;
                }
                subarraySum(&sum);
            }
        }
        ans

    }
}
// @lc code=end

