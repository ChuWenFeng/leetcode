use super::Solution;
/*
 * @lc app=leetcode.cn id=668 lang=rust
 *
 * [668] 乘法表中第k小的数
 */

// @lc code=start
impl Solution {
    pub fn find_kth_number(m: i32, n: i32, k: i32) -> i32 {
        let mut left = 1;
        let mut right = m*n;
        while left < right{
            let x = left + ((right - left)/2);
            let mut count = x/n*n;
            for i in (x/n)+1 ..= m{
                count += x/i;
            }
            if count >= k{
                right = x;
            }else{
                left = x+1;
            }
        }

        return left;
    }
}
// @lc code=end

/*
    1   2   3   4   5   6   7   8
    2   4   6   8   10  12  14  16
    3   6   9   12  15  18  21  24
    4   8   12  16  20  24  28  32
    5   10  15  20  25  30  35  40
    6   12  18  24  30  36  42  48
    7   14  21  28  35  42  49  26
    8   16  24  32  40  48  56  64
*/