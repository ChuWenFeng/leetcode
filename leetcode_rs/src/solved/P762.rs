use  super::Solution;

/*
 * @lc app=leetcode.cn id=762 lang=rust
 *
 * [762] 二进制表示中质数个计算置位
 */

// @lc code=start
impl Solution {
    pub fn count_prime_set_bits(left: i32, right: i32) -> i32 {
        let mut count = 0;
        let mut primelist = vec![true;32];
        primelist[0] = false;
        primelist[1] = false;
        for i in 2..32{
            if primelist[i]{
                let mut j = i*i;
                while j < 32{
                    primelist[j] = false;
                    j+=i;
                }
            }
        }
        let count_bits = |mut x:i32|->usize{
            let mut count = 0;
            while x >0{
                x &= x-1;
                count+=1;
            }
            return count;
        };
        for i in left..=right{
            if primelist[count_bits(i)]{
                count+=1;
            }
        }
        return count;
    }
}
// @lc code=end

