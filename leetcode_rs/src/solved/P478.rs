// use super::Solution;
/*
 * @lc app=leetcode.cn id=478 lang=rust
 *
 * [478] 在圆内随机生成点
 */

// @lc code=start
use rand::{distributions::{Distribution, Uniform}, prelude::ThreadRng};

struct Solution {
    x:f64,
    y:f64,
    r:f64,
    rng:ThreadRng,
    unifrom:Uniform<f64>,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Solution {

    fn new(radius: f64, x_center: f64, y_center: f64) -> Self {
        Solution { x: x_center, y: y_center, r: radius ,rng:rand::thread_rng(),unifrom:Uniform::from(-radius..=radius)}
    }
    
    fn rand_point(&mut self) -> Vec<f64> {
        // let mut rng = rand::thread_rng();
        // let die = Uniform::from(-self.r..=self.r);
        loop{
            let (x,y) = (self.unifrom.sample(&mut self.rng),self.unifrom.sample(&mut self.rng));
            if x*x + y*y <= self.r*self.r{
                return vec![x+self.x,y+self.y];
            }
        }
    }
}

/*
 * Your Solution object will be instantiated and called as such:
 * let obj = Solution::new(radius, x_center, y_center);
 * let ret_1: Vec<f64> = obj.rand_point();
 */
// @lc code=end

