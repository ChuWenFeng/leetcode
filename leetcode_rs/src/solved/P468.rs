use std::u8;

use super::Solution;
/*
 * @lc app=leetcode.cn id=468 lang=rust
 *
 * [468] 验证IP地址
 */

// @lc code=start
use std::net::{Ipv4Addr,Ipv6Addr};
impl Solution {
    pub fn valid_ip_address(query_ip: String) -> String {
        let mut ipv4:Vec<&str> = query_ip.split('.').collect();
        let mut ipv6:Vec<&str> = query_ip.split(':').collect();
        
        if ipv4.len() == 4{
            for str in ipv4{
                if !valid_ipv4(str){
                    return "Neither".into();
                }
                
            }
            return "IPv4".into()
        }else if ipv6.len() == 8{
            for str in ipv6{
                if !valid_ipv6(str){
                    return "Neither".into();
                }
            }
            return "IPv6".into()
        }
        


        return "Neither".into();
    }
}

fn valid_ipv4(s:&str)->bool{
    if s.len()>3 || s.len() == 0{
        return false;
    }
    let mut num = 0;
    let mut first_z = false;
    for (i,c) in s.bytes().enumerate(){
        if c >= '0' as u8 && c <= '9' as u8{
            num = num*10 + c as i32 - '0' as i32;
            if i == 0 && c == '0' as u8{
                first_z = true;
            }
        }else{
            return false;
        }
    }

    if s.len() > 1 && first_z || num>255{
        return false;
    }

    return true;
}

fn valid_ipv6(s:&str)->bool{
    if s.len()>4 || s.len()==0{
        return false
    }
    for c in s.chars(){
        match c{
            '0'..='9' =>{

            },
            'a'..='f'=>{

            },
            'A'..='F'=>{

            },
            _=>{
                return false;
            }
        }
    }


    return true;
}
// @lc code=end

#[test]
fn test(){
    let query_ip = "1.0.1.".into();
    let ans = "Neither".to_string();
    let res = Solution::valid_ip_address(query_ip);
    assert_eq!(res,ans);
}