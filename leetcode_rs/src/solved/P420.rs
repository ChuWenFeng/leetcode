use super::Solution;

/*
 * @lc app=leetcode.cn id=420 lang=rust
 *
 * [420] 强密码检验器
 */

// @lc code=start
impl Solution {
    pub fn strong_password_checker(password: String) -> i32 {
        let (mut lower,mut upper, mut digit) = (0,0,0);

        for i in password.chars(){
            match i {
                'a'..='z' =>{
                    lower=1;
                },
                'A'..='Z'=>{
                    upper=1;
                },
                '0'..='9'=>{
                    digit=1;
                },
                _ => continue,
            }
        }
        let max = |a,b|->usize{
            if a>b {a}else{b}
        };
        let min = |a,b|->usize{
            if a<b {a}else{b}
        };

        let categories = lower+upper+digit;

        match password.len() {
            len if len <6 =>{
                return max(6-len,3-categories) as i32
            },
            len @ 6..=20 =>{
                let (mut replace,mut cnt,mut cur) = (0,0,'#');
                for c in password.chars(){
                    if c == cur{
                        cnt+=1;
                    }else{
                        replace += cnt/3;
                        cnt = 1;
                        cur = c;
                    }
                }
                replace += cnt/3;
                return max(replace,3-categories) as i32
            },
            len if len > 20 => {
                let (mut replace,mut remove) = (0,len-20);
                let (mut rm,mut cnt,mut cur) = (0,0,'#');
                for c in password.chars(){
                    if c == cur{
                        cnt+=1;
                        continue;
                    }
                    if remove > 0 && cnt >= 3{
                        match cnt % 3 {
                            n if n==0 =>{
                                remove -=1;
                                replace -=1;
                            },
                            n if n==1 =>{
                                rm +=1;
                            },
                            _ => {},
                        }
                    }
                    replace += cnt/3;
                    cnt = 1;
                    cur = c;
                }

                if remove > 0 && cnt >= 3{
                    if cnt%3 == 0{
                        remove -=1;
                        replace -=1;
                    }else if cnt%3 == 1{
                        rm+=1;
                    }
                }
                replace += cnt/3;

                

                let mut u = min(min(replace,rm),remove/2);

                replace -= u;
                remove -= u *2;

                u = min(replace,remove/3);
                replace -= u;
                remove -= u*3;
                return ((len-20) + max(replace,3-categories)) as i32;
            },
            _ => {},
        }

        return 0;
    }
}
// @lc code=end

