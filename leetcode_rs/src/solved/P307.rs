use super::Solution;

/*
 * @lc app=leetcode.cn id=307 lang=rust
 *
 * [307] 区域和检索 - 数组可修改
 */

// @lc code=start
struct NumArray {
    nums:Vec<i32>,
    tree:Vec<i32>,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NumArray {

    fn new(nums: Vec<i32>) -> Self {
        let tree = vec![0;nums.len()+1];
        let mut numarry = NumArray { nums:nums.clone() ,tree};
        for (idx,val) in nums.iter().enumerate(){
            numarry.add(idx+1, *val);
        }
        numarry
    }
    fn add(&mut self,mut idx:usize,val :i32){
        while idx<self.tree.len(){
            self.tree[idx] += val;

            idx += idx & (0-idx);
        }
    }
    
    fn update(&mut self, index: i32, val: i32) {
        let idx = index as usize;   
        self.add(idx +1, val - self.nums[idx]);
        self.nums[idx] = val;
    }
    
    fn sum_range(&self, left: i32, right: i32) -> i32 {
        self.prefixsum(right as usize +1) - self.prefixsum(left as usize)
    }

    fn prefixsum(&self,mut idx:usize)->i32{
        let mut sum = 0;
        while idx > 0{
            sum+=self.tree[idx];
            idx &= idx-1;
        }
        sum
    }
}

/*
 * Your NumArray object will be instantiated and called as such:
 * let obj = NumArray::new(nums);
 * obj.update(index, val);
 * let ret_2: i32 = obj.sum_range(left, right);
 */
// @lc code=end

