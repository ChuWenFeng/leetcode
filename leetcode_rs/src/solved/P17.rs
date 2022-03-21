use super::Solution;

/*
 * @lc app=leetcode.cn id=17 lang=rust
 *
 * [17] 电话号码的字母组合
 */

// @lc code=start
impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        let mut ans = vec![];
        if digits.len() == 0{
            return ans;
        }

        let button = ["","","abc","def","ghi","jkl","mno","pqrs","tuv","wxyz"];
        let button:Vec<String> = button.iter().map(|str|str.to_string()).collect();
        let mut stack = String::new();
        let digitvec:Vec<usize> = digits.chars().map(|c| c as usize - '0' as usize).collect();
        

        fn abc_combin(but:&Vec<String>,nums:&Vec<usize>,idx:usize,stack:&mut String,ans:&mut Vec<String>){
            if idx == nums.len(){
                ans.push(stack.clone());
                return;
            }
            for i in but[nums[idx]].chars(){
                stack.push(i);
                abc_combin(but, nums, idx+1, stack, ans);
                stack.pop();
            }
        }
        
        abc_combin(&button, &digitvec, 0, &mut stack,&mut ans);
        

        return ans;
    }
}
// @lc code=end

#[test]
fn test(){
    let input = "23".to_string();
    let ans:Vec<String> = ["ad","ae","af","bd","be","bf","cd","ce","cf"]
                            .iter().map(|str|str.to_string()).collect();

    let res = Solution::letter_combinations(input);
    assert_eq!(res,ans);
}