use super::Solution;
/*
 * @lc app=leetcode.cn id=691 lang=rust
 *
 * [691] 贴纸拼词
 */

// @lc code=start
impl Solution {
    pub fn min_stickers(stickers: Vec<String>, target: String) -> i32 {
        let m = target.len();
        let mut f = vec![-1;1<<m];
        f[0] = 0;

        let mut target:Vec<char> = target.chars().collect();
        let mut ss = vec![vec![0;26]];
        for s in stickers.iter(){
            let mut cnt = vec![0;26];
            for c in s.chars(){
                cnt[c as usize - 'a' as usize]+=1;
            }
            ss.push(cnt);
        }
        let mut ans = dp((1<<m) -1,&mut f,&ss,&target);
        if ans <= m as i32{
            return ans;
        }
        return -1;
    }
}
fn dp(mask:usize,f:&mut Vec<i32>,stickers:&Vec<Vec<i32>>,target:&Vec<char>)->i32{

    if f[mask] != -1{
        return f[mask];
    }
    f[mask]=target.len() as i32 +1;
    for sticker in stickers{
        let mut left = mask;
        let mut cnt = sticker.clone();
        for (i,&c) in target.iter().enumerate(){
            let idx = c as usize - 'a' as usize;
            if mask>>i & 1 == 1 && cnt[idx] > 0{
                cnt[idx]-=1;
                left^=1<<i;
            }
        }
        if left < mask{
            f[mask] = f[mask].min(dp(left, f, stickers, target)+1)
        }
    }

    return f[mask];
}
// @lc code=end

#[test]
fn test(){
    let stickers:Vec<String> = ["with","example","science"].iter().map(|str|str.to_string()).collect();
    let target = "thehat".to_string();
    let ans = 3;
    let res = Solution::min_stickers(stickers, target);
    assert_eq!(res,ans);
}