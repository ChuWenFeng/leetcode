use super::Solution;
/*
 * @lc app=leetcode.cn id=282 lang=rust
 *
 * [282] 给表达式添加运算符
 */

// @lc code=start
impl Solution {
    pub fn add_operators(num: String, target: i32) -> Vec<String> {
        let len = num.len();
        let mut ans = vec![];
        let num = num.chars().collect();

        bt(&num, 0, 0, 0, &mut vec![], target as i64, &mut ans);

        return ans;
    }
}

fn bt(num:&Vec<char>,i:usize,res:i64,mul:i64,expr:&mut Vec<char>,target:i64,ans:&mut Vec<String>){
    if i == num.len(){
        if res == target{
            let mut s = String::new();
            expr.iter().fold(&mut s, |s,&c|{
                s.push(c);
                s
            });
            ans.push(s);
        }
        return;
    }
    let signindex = expr.len();
    if i > 0{
        expr.push('0');
    }
    let mut j = i;
    let mut val = 0;
    while j<num.len() && (j==i || num[i]!='0') {
        val = val*10+ (num[j] as i64 -'0' as i64);
        expr.push(num[j]);
        if i == 0{
            bt(num, j+1, val, val,&mut expr.clone(), target, ans);
        }else{
            expr[signindex] = '+';
            bt(num, j+1, res+val, val, &mut expr.clone(), target, ans);
            expr[signindex] = '-';
            bt(num, j+1, res-val, -val, &mut expr.clone(), target, ans);
            expr[signindex] = '*';
            bt(num, j+1, res-mul+mul*val, mul*val, &mut expr.clone(), target, ans)
        }
        j+=1;
    }
}
// @lc code=end

#[test]
fn test(){
    let num = "123".to_string();
    let target = 6;
    let ans:Vec<String> = ["1*2*3","1+2+3"].iter().map(|x|x.to_string()).collect();
    let res = Solution::add_operators(num, target);
    assert_eq!(res,ans);
}