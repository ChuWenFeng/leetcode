use super::Solution;


use std::collections::VecDeque;
impl Solution {
    pub fn alien_order(words: Vec<String>) -> String {
        let mut ans = String::new();
        let mut list = vec![[false;26];26];
        let mut inline = vec![0;26];
        let mut vis = vec![false;26];
        let mut cnt = 0;
        let mut build = |a:&String,b:&String|->bool{
            let n = a.len();
            let m = b.len();
            let len = n.min(m);
            for (c1,c2) in a.chars().map(|x|x as usize - 'a' as usize).zip(b.chars().map(|x|x as usize - 'a' as usize)){
                if c1 != c2 {
                    if list[c1][c2]{
                        return true;
                    }
                    inline[c2]+=1;
                    list[c1][c2] = true;
                    return true;
                }
            }
            return n<=m;
        };
        for (i,word) in words.iter().enumerate(){
            for c in word.chars(){
                if !vis[c as usize - 'a' as usize]{
                    cnt+=1;
                    vis[c as usize - 'a' as usize] = true;
                }
            }
            for j in 0..i{
                if !build(&words[j],word){
                    return "".to_string();
                }
            }
        }
        let mut queue = VecDeque::new();

        for (i,&v) in inline.iter().enumerate(){
            if vis[i] && v == 0{
                queue.push_back(i);
            }
        }

        while !queue.is_empty() {
            let u = queue.pop_front().unwrap();
            ans.push((u as u8 + 'a' as u8) as char);
            for (i,&out) in list[u].iter().enumerate(){
                if out {
                    inline[i]-=1;
                    if inline[i] == 0{
                        queue.push_back(i);
                    }
                }
            }
        }

        if ans.len() == cnt{
            return ans;
        }else{
            return "".into();
        }
    }
}

#[test]
fn test(){
    let words:Vec<String> = ["wrt","wrf","er","ett","rftt"].iter().map(|x|x.to_string()).collect();
    let ans = "wertf".to_string();
    let res = Solution::alien_order(words);
    assert_eq!(res,ans);
}