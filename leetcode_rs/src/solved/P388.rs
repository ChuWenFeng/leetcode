use super::Solution;

/*
 * @lc app=leetcode.cn id=388 lang=rust
 *
 * [388] 文件的最长绝对路径
 */

// @lc code=start
impl Solution {
    pub fn length_longest_path(input: String) -> i32 {
        let mut tokenstream = input.split('\n');
        let mut ans = 0;

        let mut stack:Vec<i32> = vec![];
        
        let mut currdp = 0;
        for s in tokenstream{
            let mut depth = 0;
            let mut file = false;
            let mut file_len = 0;
            for c in  s.chars(){
                if c == '\t'{
                    depth+=1;
                }else{
                    file_len+=1;
                }
                if c == '.'{
                    file = true;
                }
            }
            while depth<stack.len(){
                stack.pop();
            }
            stack.push(file_len);

            if file{
                let sum:i32 = stack.iter().sum::<i32>() + stack.len() as i32 - 1 ;
                ans = ans.max(sum);
            }
        }

        ans
    }
}
// @lc code=end

#[test]
fn test(){
    let input = "dir\n\tsubdir1\n\tsubdir2\n\t\tfile.ext".to_string();
    let ans = 20;
    let res = Solution::length_longest_path(input);
    assert_eq!(res,ans);

}