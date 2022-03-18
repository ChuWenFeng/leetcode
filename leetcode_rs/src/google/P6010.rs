use std::vec;

use super::Solution;

/*
 * @lc app=leetcode.cn id=6010 lang=rust
 *
 * [6010] 完成旅途的最少时间
 */

// @lc code=start
impl Solution {
    pub fn minimum_time(time: Vec<i32>, total_trips: i32) -> i64 {
        
        let mut min = i32::MAX;
        let mut max = 0;
        
        let check = |ans:i64|->bool{
            let mut count = 0;
            for &val in time.iter(){
                count += ans/(val as i64);
                if count as i32 >= total_trips{
                    return true;
                }
            }
            return count as i32 >= total_trips;
        };
        
        for &val in time.iter(){
            if val < min{
                min = val;
            }
            if val > max{
                max = val;
            }
        }
        let mut left= 0;
        let mut right = min as i64 * total_trips as i64;

        while left < right{
            let mid = (left + right) /2;
            if check(mid){
                right = mid;
            }else{
                left = mid +1;
            }
        }
        return left;
    }
}
// @lc code=end

#[test]
fn test(){
    let v = vec![10000];
    let total = 10000000;
    let result= Solution::minimum_time(v, total);
    assert_eq!(result,100000000000);
}