use super::Solution;
/*
 * @lc app=leetcode.cn id=214 lang=rust
 *
 * [214] 最短回文串
 */

// @lc code=start
impl Solution {
    pub fn shortest_palindrome_1(s: String) -> String {//朴素的算法，一个一个检查
        let mut slist:Vec<char> = s.chars().collect();
        let mut list = String::new();
        while !slist.is_empty() {
            let len = slist.len();
            let mid = len/2;
            let mut left = 0;
            let mut right = len as i32;
            if len&1 == 1{
                left = mid as i32 -1;
                right = mid as i32 +1;
            }else{
                left = mid as i32 - 1;
                right = mid as i32;
            }
            while left>=0 && right<slist.len() as i32 && slist[left as usize] == slist[right as usize]{
                left-=1;
                right+=1;
            }
            if left <0 && right >= len as i32{
                break;
            }
            let ch = slist.pop().unwrap();
            list.push(ch);
        }
        let mut ans = String::new();

        for c in list.chars(){
            ans.push(c);
        }
        for c in slist{
            ans.push(c);
        }
        for c in list.chars().rev(){
            ans.push(c);
        }

        ans
    }

    pub fn shortest_palindrome(s:String)->String{
        let n = s.len();
        let mut slist:Vec<char> = s.chars().collect();
        let mut fail:Vec<i32> = vec![-1;n];
        for i in 1..n{
            let mut j = fail[i-1];
            while j != -1 && slist[(j+1) as usize]!= slist[i]{
                j = fail[j as usize];
            }
            if slist[(j+1) as usize] == slist[i]{
                fail[i] = j+1;
            }
        }

        let mut best = -1;

        for i in (0..n).rev(){
            while best != -1 && slist[(best+1)as usize]!= slist[i]{
                best = fail[best as usize];
            }
            if slist[(best+1) as usize] == slist[i]{
                best+=1;
            }
        }

        let mut add = String::new();
        for i in ((best+1) as usize..n).rev(){
            add.push(slist[i]);
        }
        add.push_str(&s);


        return add;
    }
}
// @lc code=end
//a     a   c   e   c   a   a   a
//-1    0   -1  -1  
#[test]
fn test(){
    let s = "aacecaaa".to_string();
    let ans = "aaacecaaa".to_string();
    let res = Solution::shortest_palindrome(s);
    assert_eq!(res,ans);
}