use super::Solution;
/*
 * @lc app=leetcode.cn id=240 lang=rust
 *
 * [240] 搜索二维矩阵 II
 */

// @lc code=start
impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {

        let n = matrix.len();
        if n == 0{
            return false;
        }
        let m = matrix[0].len();
        if m == 0{
            return false;
        }

        let mut row = 0;
        let mut col = m-1;

        while row < n && col >= 0 {
            match matrix[row][col].cmp(&target){
                 std::cmp::Ordering::Less =>{
                    row +=1;
                },
                 std::cmp::Ordering::Greater =>{
                    if col == 0{
                        return false;
                    }
                    col -=1;
                },
                _ => return true,
            }
            
        }
        

        return false;
    }
}
// @lc code=end

