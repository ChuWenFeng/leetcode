

use super::Solution;
/*
 * @lc app=leetcode.cn id=336 lang=rust
 *
 * [336] 回文对
 */

// @lc code=start
use std::{collections::HashMap, hash::Hash};
impl Solution {
    pub fn palindrome_pairs(words: Vec<String>) -> Vec<Vec<i32>> {// time limit exceeded 135/136
        let mut wordsRev:Vec<String> = vec![];
        let mut indices:HashMap<&str, usize> = HashMap::new();
        let n = words.len();
        for word in words.iter(){
            let rev = word.chars().rev().collect();
            wordsRev.push(rev);
        }
        
        for i in 0..n{
            let s = &wordsRev[i][..];
            indices.insert(s, i);
        }

        let mut isPalindrome = |s:&Vec<char>,left:usize,right:usize|->bool{
            for i in 0..(right-left+1)/2{
                if s[left+i] != s[right-i]{
                    return false;
                }
            }
            return true;
        };
        let mut findword = |word:&String,left:usize,right:usize|->(usize,bool){
            // let s = word[left..right+1];
            if let Some(&v) = indices.get(&word[left..right+1]){
                return (v,true);
            }
            return (0,false);
        };

        let mut ans = vec![];

        for i in 0..n{
            let word = &words[i];
            let wordlist:Vec<char> = word.chars().collect();
            let m = word.len();
            if m == 0{
                continue;
            }
            for j in 0..=m{
                if isPalindrome(&wordlist,j,m-1){
                    let left_id = findword(word,0,j-1);
                    if left_id.1 && left_id.0 != i{
                        ans.push(vec![i as i32,left_id.0 as i32]);
                    }
                }
                if j != 0 && isPalindrome(&wordlist,0,j-1){
                    let right_id = findword(word,j,m-1);
                    if right_id.1 && right_id.0 != i{
                        ans.push(vec![right_id.0 as i32,i as i32]);
                    }
                }
            }
        }

        ans
    }
}
// fn findword(s:&String,left:usize,right:usize,indices:HashMap<&str,usize>)->(usize,bool){

// }

// @lc code=end

