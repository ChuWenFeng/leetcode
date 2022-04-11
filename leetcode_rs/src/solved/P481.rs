use super::Solution;
/*
 * @lc app=leetcode.cn id=481 lang=rust
 *
 * [481] 神奇字符串
 */

// @lc code=start
impl Solution {
    pub fn magical_string(n: i32) -> i32 {
        if n <= 3{
            return 1;
        }
        
        let mut str = vec![1,2,2];
        let mut p = str.len()-1;

        let mut len = 2;

        while len+1 < n as usize{
            let sn = str[p];
            match str[len]{
                1 => {
                    for _ in 0..sn{
                        str.push(2);
                        len+=1;
                    }
                },
                2 =>{
                    for _ in 0..sn{
                        str.push(1);
                        len+=1;
                    }
                },
                _ =>{},
            }
            p+=1;
        }
        let mut count = 0;
        for i in 0..n as usize{
            if str[i] == 1{
                count+=1;
            }
        }

        return count;
    }
}
// @lc code=end

