use core::time;


use super::Solution;

/*
 * @lc app=leetcode.cn id=1606 lang=rust
 *
 * [1606] 找到处理最多请求的服务器
 */

// @lc code=start
impl Solution {
    pub fn busiest_servers(k: i32, arrival: Vec<i32>, load: Vec<i32>) -> Vec<i32> {
        use std::cmp::Ordering;
        use std::collections::BinaryHeap;
        use std::cmp::Reverse;
  
        let mut idle = BinaryHeap::new();
        let mut busy = BinaryHeap::new();
        let k = k as usize;
        (0..k).for_each(|i|idle.push(Reverse(i)));

        // let mut available = vec![0;k];
        let mut requist = vec![0;k];

        for (idx,(&arv,&prt)) in arrival.iter().zip(load.iter()).enumerate(){
            while let Some(Reverse((time,id))) = busy.pop(){
                if time <= arv{
                    // let i = idx as i32;
                    let tmp = (id as i32 - idx as i32 ) % k as i32 +k as i32 as i32;//?
                    let next = idx + (tmp as usize)%k;//?
                    idle.push(Reverse(next));
                }else{
                    busy.push(Reverse((time,id)));
                    break;
                }
            }

            if let Some(Reverse(id)) = idle.pop(){
                let id = id % k;
                requist[id]+=1;
                busy.push(Reverse((arv+ prt,id)));
            }
        }

        let mut ans = vec![];
        let mut max = 0;
        for (idx,&count) in requist.iter().enumerate(){
            let idx = idx as i32;
            match count.cmp(&max) {
                Ordering::Less=>{},
                Ordering::Equal=>{
                    ans.push(idx);
                },
                Ordering::Greater=>{
                    max = count;
                    ans.clear();
                    ans.push(idx);
                },
            }
        }

        return ans;
    }
}
// @lc code=end


#[test]
fn test(){
    let input1 = 3;
    let input2 = vec![1,2,3,4,5];
    let input3 = vec![5,2,3,3,3];
    let ans = vec![1];
    let res = Solution::busiest_servers(input1, input2, input3);

    assert_eq!(res,ans);
    let k:i32 = 1;
    let i:usize = 3;    
}  
