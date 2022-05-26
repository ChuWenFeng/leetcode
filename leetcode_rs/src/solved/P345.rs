use super::Solution;
/*
 * @lc app=leetcode.cn id=345 lang=rust
 *
 * [345] 反转字符串中的元音字母
 */

// @lc code=start
impl Solution {
    pub fn reverse_vowels(s: String) -> String {
        let mut list = s.chars().collect::<Vec<char>>();

        let mut left = 0;
        let mut right = s.len() as i32 -1;

        while left<s.len() as i32 && right>=0 && left<right{

            while  right>=0 && left<right  {
                match list[right as usize]{
                    'a'|'e'|'i'|'o'|'u'|'A'|'E'|'I'|'O'|'U'=>{
                        break;
                    },
                    _=>{
                        right-=1;
                    }
                }
            }
            while left<s.len() as i32 && left<right  {
                match list[left as usize]{
                    'a'|'e'|'i'|'o'|'u'|'A'|'E'|'I'|'O'|'U'=>{
                        break;
                    },
                    _=>{
                        left+=1;
                    }
                }
            }
            if left<right{
                let tmp = list[left as usize];
                list[left as usize] = list[right as usize];
                list[right as usize] = tmp;
                left+=1;
                right-=1;
            }

        }
        
        let mut ans = String::new();
        ans.extend(list.iter());
        ans
    }
}
// @lc code=end

#[test]
fn test(){
    let s = "hello".to_string();
    let ans = "holle".to_string();
    let res = Solution::reverse_vowels(s);
    assert_eq!(res,ans);
}