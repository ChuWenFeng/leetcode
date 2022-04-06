use super::Solution;

/*
 * @lc app=leetcode.cn id=60 lang=rust
 *
 * [60] 排列序列
 */

// @lc code=start
impl Solution {
    pub fn get_permutation(n: i32, k: i32) -> String {
        let n = n as usize;
        let mut nlist =  vec![1;n];
        for i in 2..n{
            nlist[i] = nlist[i-1]*i;
        }
        let mut ans:usize = 0;
        let mut k = (k-1) as usize;
        let mut flag = vec![1;n];

        for i in (0..=n-1).rev(){
            let m = nlist[i];
            let mut a1 = k / m +1;
            for (idx,fl) in flag.iter_mut().enumerate(){
                a1 -= *fl;
                if a1 == 0{
                    ans = ans*10 + idx + 1;
                    *fl = 0;
                    break;
                }
            }
            k = k % m;
        }
        return ans.to_string();
    }
}
// @lc code=end

#[test]
fn test(){
    let n = 3;
    let k = 3;
    let ans = "213".to_string();
    let res = Solution::get_permutation(n, k);
    assert_eq!(res,ans);
}