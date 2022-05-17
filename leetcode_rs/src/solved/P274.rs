use super::Solution;
/*
 * @lc app=leetcode.cn id=274 lang=rust
 *
 * [274] H 指数
 */

// @lc code=start
impl Solution {
    pub fn h_index(citations: Vec<i32>) -> i32 {
        let len = citations.len();
        let mut count:Vec<usize> = vec![0;len+1];
        for c in citations{
            let c = c as usize;
            if c >= len{
                count[len]+=1;
            }else{
                count[c]+=1;
            }
        }

        let mut cnt = 0;
        for (i,&c) in count.iter().enumerate().rev(){
            cnt+=c;
            if cnt >= i{
                return i as i32;
            }
        }

        0
    }
}
// @lc code=end

