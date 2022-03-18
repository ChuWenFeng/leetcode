struct Solution{

}

impl Solution {
    fn new() -> Self {
        Solution{}
    }

    /**
    * 代码中的类名、方法名、参数名已经指定，请勿修改，直接返回方法规定的值即可
    * 
        * @param num int整型一维数组 
        * @return int整型二维数组
    */
    pub fn permute(&self, num: Vec<i32>) -> Vec<Vec<i32>> {
        use std::collections::VecDeque;
        let len = num.len();
        let mut num:VecDeque<i32> = num.iter().map(|x|*x).collect();
        let mut ans = vec![];
        let mut tmp = vec![];
        fn FB(num:&mut VecDeque<i32>,idx:usize,ans:&mut Vec<Vec<i32>>,tmp:&mut Vec<i32>){
            if idx == 0{
                ans.push(tmp.clone());
            }
            for i in 0..idx{
                tmp.push(num.pop_front().unwrap());
                FB(num, idx-1, ans, tmp);
                num.push_back(tmp.pop().unwrap());
            }
        }
        FB(&mut num,len,&mut ans,&mut tmp);
        ans
    }
}