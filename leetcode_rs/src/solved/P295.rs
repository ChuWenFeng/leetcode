use super::Solution;
/*
 * @lc app=leetcode.cn id=295 lang=rust
 *
 * [295] 数据流的中位数
 */

// @lc code=start
use std::{collections::BinaryHeap, cmp::Reverse};
struct MedianFinder {
    Min:BinaryHeap<i32>,
    Max:BinaryHeap<Reverse<i32>>,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MedianFinder {

    fn new() -> Self {
        MedianFinder { Min: BinaryHeap::new(), Max: BinaryHeap::new() }
    }
    
    fn add_num(&mut self, num: i32) {
        if self.Min.is_empty() || *self.Min.peek().unwrap()>= num{
            self.Min.push(num);
            if self.Max.len()+1 < self.Min.len(){
                let vm = self.Min.pop().unwrap();
                self.Max.push(Reverse(vm));
            }
        }else{
            self.Max.push(Reverse(num));
            if self.Max.len()>self.Min.len(){
                let vm = self.Max.pop().unwrap();
                self.Min.push(vm.0);
            }
        }
    }
    
    fn find_median(&self) -> f64 {
        if self.Min.len()>self.Max.len(){
            return *self.Min.peek().unwrap() as f64;
        }else{
            return (*self.Min.peek().unwrap() as f64 + (self.Max.peek().unwrap().0) as f64)/2.0;
        }
    }
}

/*
 * Your MedianFinder object will be instantiated and called as such:
 * let obj = MedianFinder::new();
 * obj.add_num(num);
 * let ret_2: f64 = obj.find_median();
 */
// @lc code=end

