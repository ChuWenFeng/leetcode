use super::Solution;
/*
 * @lc app=leetcode.cn id=2055 lang=rust
 *
 * [2055] 蜡烛之间的盘子
 */

// @lc code=start
impl Solution {
    pub fn plates_between_candles(s: String, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let len = s.len();
        let mut presum = vec![0;len];//.0表示蜡烛的前缀和，.1表示盘子的前缀和
        let mut right_candle = vec![len;len];
        let mut left_candle = vec![len;len];
        let mut ans = vec![];
        let mut pre_candle = 0;
        let mut pre_plate = 0;
        let skip = if let Some(i) = s.find('|'){
            i
        }else{
            len
        };
        let sc:Vec<char> = s.chars().collect();
        for (idx,c) in sc.iter().enumerate().skip(skip+1){
            match c {
                '*'=>{
                    pre_plate+=1;
                },
                '|'=>{
                    presum[idx] = pre_plate;
                }
                _=>{},
            }
        }
        let mut plant_count = 0;
        for (idx,&c) in sc.iter().enumerate(){
            match c{
                '*'=> plant_count+=1,
                '|'=>{
                    for i in 0..=plant_count{
                        right_candle[idx-i] = idx;
                    }
                    plant_count = 0;
                },
                _=>{},
            }
        }
        plant_count = 0;
        for (idx,&c) in sc.iter().enumerate().rev(){
            match c{
                '*'=>plant_count+=1,
                '|'=>{
                    for i in 0..=plant_count{
                        left_candle[idx+i] = idx;
                    }
                    plant_count = 0;
                },
                _=>{},
            }
        }

        for subq in queries{
            let left = right_candle[subq[0] as usize];
            let right = left_candle[subq[1] as usize];

            if left == len || right == len || left >= right{
                ans.push(0);
                continue;
            }
            ans.push(presum[right] - presum[left]);

        }

        return ans;
    }
}
// @lc code=end

