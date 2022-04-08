use super::Solution;
/*
 * @lc app=leetcode.cn id=1652 lang=rust
 *
 * [1652] 拆炸弹
 */

// @lc code=start
impl Solution {
    pub fn decrypt(code: Vec<i32>, k: i32) -> Vec<i32> {
        let len = code.len();
        if k == 0{
            return vec![0;len]
        }
        let mut code = code.clone();
        code.append(&mut code.clone());
        let mut ans = vec![0;len];
        let mut idx;
        let mut flag = 0;
        if k > 0{
            idx = 0;
            flag = 1;
        }else{
            idx = len as i32;
        }
        let mut count = 0;
        while count < len{

            let i = idx+k;
            let left = i.min(idx)+flag;
            let right = i.max(idx)+flag;
            let mut sum = 0;
            for idx in left..right{
                sum+= code[idx as usize];
            }
            idx+=1;
            ans[count] = sum;
            count +=1;
        }
        
        

        ans
    }
}
// @lc code=end

