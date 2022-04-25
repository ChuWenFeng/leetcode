use super::Solution;

/*
 * @lc app=leetcode.cn id=398 lang=rust
 *
 * [398] 随机数索引
 */

// @lc code=start
use rand::{self, Rng};
struct _Solution {
    r : rand::rngs::ThreadRng,
    nums: Vec<i32>,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */

impl _Solution {

    fn new(nums: Vec<i32>) -> Self {
        _Solution { r: rand::thread_rng(), nums: nums }
    }
    
    fn pick(&mut self, target: i32) -> i32 {
        let mut cnt:usize = 0;
        let mut ans = 0;
        for (idx,&v) in self.nums.iter().enumerate(){
            if v == target{
                cnt+=1;
                if self.r.gen_range(0,cnt) == 0{
                    ans = idx;
                }
            }
        }
        return ans as i32;
    }
}

/*
 * Your Solution object will be instantiated and called as such:
 * let obj = Solution::new(nums);
 * let ret_1: i32 = obj.pick(target);
 */
// @lc code=end

