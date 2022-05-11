use super::Solution;
/*
 * @lc app=leetcode.cn id=212 lang=rust
 *
 * [212] 单词搜索 II
 */

// @lc code=start
#[derive(Default)]
struct Trie {
    child:[Option<Box<Trie>>;26],
    isword:bool,
}
const dirs:[(i32,i32);4] = [(1,0),(-1,0),(0,1),(0,-1)];
impl Trie{
    fn insert(&mut self, word: String) {
        let mut curr = self;
        for c in word.chars(){
            let idx = c as usize - 'a' as usize;
            curr = curr.child[idx].get_or_insert(Box::new(Self::default()));
        }
        curr.isword = true;
    }
}

impl Solution {
    pub fn find_words(board: Vec<Vec<char>>, words: Vec<String>) -> Vec<String> {
        let mut trie = Trie::default();
        let mut ans = vec![];
        for word in words.iter(){
            trie.insert(word.clone());
        }
        let mut mutboard = board.clone();
        let mut root = Box::new(trie);
        for (i,row) in board.iter().enumerate(){
            for j in 0..row.len(){
                dfs(&mut root, i as i32, j as i32, &mut ans,&mut "".to_string(), &mut mutboard)
            }
        }

        ans
    }
}

fn dfs(node:&mut Box<Trie>,x:i32,y:i32,ans:&mut Vec<String>,currstr:&mut String,board:&mut Vec<Vec<char>>){
    let ch = board[x as usize][y as usize];
    let next = node.child[ch as usize - 'a' as usize].as_mut();
    if let Some(nextnode) = next{
        currstr.push(ch);
        if nextnode.isword{
            ans.push(currstr.clone());
            nextnode.isword = false;
        }
        if nextnode.child.iter().fold(0, |count,x|{
            if x.is_some(){
                return count+1;
            }
            count
        }) > 0{
            board[x as usize][y as usize] = '#';
            for dir in dirs{
                let nx = x + dir.0;
                let ny = y + dir.1;
                if nx >= 0 && nx < board.len() as i32 && ny >= 0 && ny < board[0].len() as i32 && board[nx as usize][ny as usize] != '#'{
                    dfs(nextnode, nx, ny, ans, currstr, board);
                }
            }
            board[x as usize][y as usize] = ch;
        }else{
            node.child[ch as usize - 'a' as usize] = None;
        }
        currstr.pop();
    }else{
        return;
    }

}
// @lc code=end

#[test]
fn test(){
    let board:Vec<Vec<char>> = [["o","a","b","n"],["o","t","a","e"],["a","h","k","r"],["a","f","l","v"]].iter().map(|strs|strs.iter().map(|str|str.chars().next().unwrap()).collect()).collect();
    let words:Vec<String> = ["oa","oaa"].iter().map(|str|str.to_string()).collect();
    let ans :Vec<String>= ["oa","oaa"].iter().map(|str|str.to_string()).collect();
    let res = Solution::find_words(board, words);
    for s in res.iter(){
        assert!(ans.contains(s));
    }
    assert_eq!(res,ans);

}