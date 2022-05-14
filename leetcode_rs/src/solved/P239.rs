use super::Solution;
/*
 * @lc app=leetcode.cn id=239 lang=rust
 *
 * [239] 滑动窗口最大值
 */

// @lc code=start
struct pair{
    val:i32,
    idx:usize,
}
impl Ord for pair{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.val.cmp(&other.val){
            Ordering::Equal=>{
                match self.idx.cmp(&other.idx) {
                    Ordering::Equal=>{return Ordering::Equal}
                    Ordering::Greater=>{return Ordering::Less}
                    Ordering::Less=>{return Ordering::Greater}
                }
            },
            other =>{
                return other;
            }
        }
    }
}

impl PartialOrd for pair{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.val.partial_cmp(&other.val) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        self.idx.partial_cmp(&other.idx)
    }
}

impl Eq for pair{

}

impl PartialEq for pair{
    fn eq(&self, other: &Self) -> bool {
        self.val == other.val && self.idx == other.idx
    }
}

use std::{collections::BinaryHeap, cmp::Ordering};
use std::collections::VecDeque;
impl Solution {
    pub fn max_sliding_window_1(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut heap:BinaryHeap<pair> = BinaryHeap::new();
        let mut ans = vec![];
        for i in 0..k as usize{
            heap.push(pair { val: nums[i], idx: i });
        }
        ans.push(heap.peek().unwrap().val);
        for i in k as usize .. nums.len(){
            while !heap.is_empty() && heap.peek().unwrap().idx<= i-k as usize{
                heap.pop();
            }
            heap.push(pair { val: nums[i], idx: i });
            ans.push(heap.peek().unwrap().val);
        }

        ans
    }

    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut queue:VecDeque<usize> = VecDeque::new();
        let mut ans = vec![];
        for i in 0..k as usize{
            while !queue.is_empty() && nums[i]>nums[*queue.back().unwrap()]{
                queue.pop_back();
            }
            queue.push_back(i);
        }

        ans.push(nums[*queue.front().unwrap()]);

        for i in k as usize .. nums.len(){
            while !queue.is_empty() && *queue.front().unwrap() <= i-k as usize{
                queue.pop_front();
            }

            while !queue.is_empty() && nums[i]>nums[*queue.back().unwrap()] {
                    queue.pop_back();
            }
            queue.push_back(i);
            ans.push(nums[*queue.front().unwrap()])

        }
        ans
    }
}
// @lc code=end

