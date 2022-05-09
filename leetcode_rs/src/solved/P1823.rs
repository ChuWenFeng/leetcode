use super::Solution;
/*
 * @lc app=leetcode.cn id=1823 lang=rust
 *
 * [1823] 找出游戏的获胜者
 */

// @lc code=start
impl Solution {
    pub fn find_the_winner_m(n: i32, k: i32) -> i32 {
        let mut idx = 0;
        let mut list = vec![0;n as usize];
        for i in 0..n{
            list[i as usize] = i+1
        }
        let k = k as usize;
        let mut len = n as usize;
        for _ in 1..n{
            idx = (idx+k-1)%len;
            for i in idx..len-1{
                list[i] = list[i+1];
            }
            len-=1;
        }

        list[0]
    }
    pub fn find_the_winner(n: i32, k: i32) -> i32 {
        let mut ans = 1;
        let mut len = 2;
        for _ in 1..n{
            ans = (ans + k - 1) % len + 1;
            len+=1;
        }

        ans
    }
}
// @lc code=end

#[test]
fn test(){
    let n = 3;
    let k = 1;
    let ans = 3;
    let res = Solution::find_the_winner(n, k);
    assert_eq!(res,ans);
}