use super::Solution;
/*
 * @lc app=leetcode.cn id=316 lang=rust
 *
 * [316] 去除重复字母
 */

// @lc code=start
impl Solution {
    pub fn remove_duplicate_letters(s: String) -> String {
        let mut count = vec![0;26];
        let mut stack:Vec<char> = vec![];
        let mut total = vec![0;26];
        s.chars().fold(&mut total, |total,c|{
            total[c as usize - 'a' as usize]+=1;
            total
        });
        for c in s.chars(){
            total[c as usize - 'a' as usize]-=1;
            if stack.is_empty(){
                stack.push(c);
                count[c as usize - 'a' as usize] +=1;
                continue;
            }
            if count[c as usize - 'a' as usize] > 0{
                continue;
            }
            while let Some(tail) = stack.last() {
                if c < *tail && total[*tail as usize - 'a' as usize]>0{
                    count[*tail as usize - 'a' as usize]-=1;
                    stack.pop();
                }else{
                    break;
                } 
            }
            stack.push(c);
            count[c as usize - 'a' as usize]+=1;
        }
        let mut ans = String::new();
        ans.extend(stack.iter());
        ans
    }
}
// @lc code=end

