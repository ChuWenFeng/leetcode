use super::Solution;
/*
 * @lc app=leetcode.cn id=929 lang=rust
 *
 * [929] 独特的电子邮件地址
 */

// @lc code=start
use std::collections::HashSet;

impl Solution {
    pub fn num_unique_emails(emails: Vec<String>) -> i32 {
        let mut table = HashSet::new();

        for email in emails.iter(){
            let s:Vec<&str> = email.split('@').collect();
            let mut localname = String::new();
            for c in s[0].chars(){
                match c {
                    '+'=>{
                        break;
                    },
                    '.'=>{

                    },
                    _=>{
                        localname.push(c);
                    }
                }
            }
            table.insert((localname,s[1]));
        }


        table.len() as i32
    }
}
// @lc code=end

