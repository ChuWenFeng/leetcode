use super::Solution;

/*
 * @lc app=leetcode.cn id=744 lang=rust
 *
 * [744] 寻找比目标字母大的最小字母
 */

// @lc code=start
impl Solution {
    pub fn next_greatest_letter(letters: Vec<char>, target: char) -> char {
        let mut list_count = vec![0;60];
        let mut ans= 0;
        let aidx = 'a' as usize;
        for &ch in letters.iter(){
            let idx = ch as usize - aidx;
            list_count[idx]+=1;
            list_count[idx+26]+=1;
        }

        let targetidx = target as usize - aidx;

        for (idx,count) in list_count.iter().enumerate().skip(targetidx+1){
            if *count > 0{
                ans = ((idx % 26) + aidx ) as u8;
                break;
            }
        }

        return ans as char;
    }
}
// @lc code=end

