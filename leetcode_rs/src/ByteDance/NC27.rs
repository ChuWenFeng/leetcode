use std::usize;

struct Solution{

}

impl Solution {
    fn new() -> Self {
        Solution{}
    }

    /**
    * 代码中的类名、方法名、参数名已经指定，请勿修改，直接返回方法规定的值即可
    * 
        * @param S int整型一维数组 
        * @return int整型二维数组
    */
    pub fn subsets(&self, S: Vec<i32>) -> Vec<Vec<i32>> {
        // write code here
        let mut ans = vec![];
        let mut s = S.clone();
        s.sort();
        let mut tmp = vec![];
        fn subs(s:&Vec<i32>,i:usize,tmp:&mut Vec<i32>,ans:&mut Vec<Vec<i32>>){
            ans.push(tmp.clone());
            for i in i..s.len(){
                tmp.push(s[i]);
                subs(s,i+1,tmp,ans);
                tmp.pop();
            }
        }
        subs(&s,0,&mut tmp,&mut ans);
        ans
    }
}