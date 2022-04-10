use super::Solution;

/*
 * @lc app=leetcode.cn id=804 lang=rust
 *
 * [804] 唯一摩尔斯密码词
 */

// @lc code=start
impl Solution {
    pub fn unique_morse_representations(words: Vec<String>) -> i32 {
        let mut hashset = std::collections::HashSet::new();
        
        let code = [".-","-...","-.-.","-..",".","..-.","--.","....","..",".---","-.-",".-..","--","-.","---",".--.","--.-",".-.","...","-","..-","...-",".--","-..-","-.--","--.."];

        for word in words{
            let mut mscode = String::new();
            for c in word.chars(){
                let idx = c as usize - 'a' as usize;
                mscode.push_str(code[idx]);
            }
            hashset.insert(mscode);
        }
          
        
        return hashset.len() as i32;

    }
}
// @lc code=end

