use super::Solution;
/*
 * @lc app=leetcode.cn id=462 lang=rust
 *
 * [462] 最少移动次数使数组元素相等 II
 */

// @lc code=start
use std::collections::BinaryHeap;
impl Solution {
    pub fn min_moves2(nums: Vec<i32>) -> i32 {
        let mut heap = BinaryHeap::new();
        let mid = nums.len()/2;
        for &v in nums.iter(){
            heap.push(v);
            if heap.len()>mid+1{
                heap.pop();
            }
        }
        let mut ans = 0;
        let target = if nums.len() & 1 == 1{
            heap.pop().unwrap()
        }else{
            heap.pop();
            heap.pop().unwrap()
        };

        for &v in nums.iter(){
            ans += (v-target).abs();
        }

        ans
    }
}
// @lc code=end

