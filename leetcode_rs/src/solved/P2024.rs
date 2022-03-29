use super::Solution;
/*
 * @lc app=leetcode.cn id=2024 lang=rust
 *
 * [2024] 考试的最大困扰度
 */

// @lc code=start
impl Solution {
    pub fn max_consecutive_answers(answer_key: String, k: i32) -> i32 {
        let len = answer_key.len();
        if len <= k as usize{
            return k;
        }
        let answer_key:Vec<char> = answer_key.chars().collect();
        let mut ans  = 0;
        let mut left = 0;
        let mut ch = answer_key[0];
        let mut max_consecutive = |ch:char|->i32{
            let mut ans  = 0;
            let mut diffnum = 0;
            let mut left = 0;
            for i in 0..len{
                if answer_key[i] != ch{
                    diffnum+=1;
                }

                while diffnum > k{
                    if answer_key[left] != ch{
                        diffnum-=1;
                    }
                    left+=1;
                }
                if i - left+1 > ans{
                    ans = i - left+1;
                }
            }
            return (ans) as i32;
         };

        let t = max_consecutive('T');
        let f = max_consecutive('F');
        if t > f{
            return t;
        }else{
            return f;
        }
    }
}
// @lc code=end

#[test]
fn test(){
    let input = "TTFTTTTTFT".to_string();
    let k = 1;
    let ans = 8;
    let res = Solution::max_consecutive_answers(input, k);
    assert_eq!(res,ans);
}