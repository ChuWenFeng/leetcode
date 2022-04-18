use super::Solution;
/*
 * @lc app=leetcode.cn id=819 lang=rust
 *
 * [819] 最常见的单词
 */

// @lc code=start
use std::{collections::HashMap, string};
impl Solution {
    pub fn most_common_word(paragraph: String, banned: Vec<String>) -> String {
        let mut map:HashMap<String, i32> = HashMap::new();
        let mut ans = "".to_string();
        let mut count = 0;
                                        
        let N_ban = paragraph.split([' ', '!', '?', '\'', ',', ';', '.']).map(|s|{
            s.to_ascii_lowercase()
        }).fold(&mut map , |map,s|{
            let c = if !s.is_empty() && !banned.contains(&s){
                let str_c = if let Some(num) = map.get_mut(&s){
                    *num+=1;
                    *num
                }else{
                    map.insert(s.clone(), 1);
                    1
                };
                str_c
            }else{
                0
            };
            if c > count{
                count = c;
                ans = s;
            }

            map
        });
        ans
    }
}
// @lc code=end

#[test]
fn test(){
    let input = "Bob. hIt, baLl".to_string();
    let banned = vec!["bob".to_string(),"hit".to_string()];
    let ans = "ball".to_string();
    let res = Solution::most_common_word(input, banned);
    assert_eq!(res,ans);
}