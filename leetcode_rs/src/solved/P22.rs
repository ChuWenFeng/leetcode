use super::Solution;
/*
 * @lc app=leetcode.cn id=22 lang=rust
 *
 * [22] 括号生成
 */

// @lc code=start
impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut left = n;
        let mut right = 0;
        let mut ans = vec![];
        let mut str = String::new();

        fn dfs(ans:&mut Vec<String>,str:&mut String,mut left:i32,mut right:i32){
            if left == 0 && right == 0{
                ans.push(str.clone());
                return;
            }
            if left > 0{
                str.push('(');
                dfs(ans,str,left-1,right+1);
                str.pop();
            }
            if right > 0{
                str.push(')');
                dfs(ans,str,left,right-1);
                str.pop();
            }
        }
        dfs(&mut ans,&mut str,left,right);

        return ans;
    }
}
// @lc code=end
#[test]
fn test(){
    let input = 3;
    let res = Solution::generate_parenthesis(input);
    println!("{:?}",res);
}
