use super::Solution;
/*
 * @lc app=leetcode.cn id=933 lang=rust
 *
 * [933] 最近的请求次数
 */

// @lc code=start
use std::collections::VecDeque;
struct RecentCounter {
    list:VecDeque<i32>
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl RecentCounter {

    fn new() -> Self {
        RecentCounter { list: VecDeque::new() }
    }
    
    fn ping(&mut self, t: i32) -> i32 {
        self.list.push_back(t);
        while *self.list.front().unwrap() < t-3000{
            self.list.pop_front();
        }
        return self.list.len() as i32
    }
}

/*
 * Your RecentCounter object will be instantiated and called as such:
 * let obj = RecentCounter::new();
 * let ret_1: i32 = obj.ping(t);
 */
// @lc code=end

