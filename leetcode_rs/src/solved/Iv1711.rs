use super::Solution;
impl Solution {
    pub fn find_closest(words: Vec<String>, word1: String, word2: String) -> i32 {
        let mut idx1 = -1;
        let mut idx2 = -1;
        let mut ans = words.len() as i32;
        for (i,word) in words.iter().enumerate(){
            if *word == word1{
                idx1 = i as i32;
                if idx2 != -1{
                    ans = ans.min((idx2-idx1).abs());
                }
            }else if *word == word2{
                idx2 = i as i32;
                if idx1 != -1{
                    ans = ans.min((idx2-idx1).abs());
                }
                
            }
        }
        ans
    }
}