use super::Solution;
/*
 * @lc app=leetcode.cn id=728 lang=rust
 *
 * [728] 自除数
 */

// @lc code=start
impl Solution {
    pub fn self_dividing_numbers(left: i32, right: i32) -> Vec<i32> {
        let mut ans = vec![];

        for v in left..=right{
            let mut num = v;
            let digit = 10;
            let mut flag = true;
            while num > 0{
                let i = num%digit;
                if i==0 || v % i != 0{
                    flag = false;
                    break;
                }
                num = num/digit;
            }

            if flag{
                ans.push(v);
            }
        }

        return ans;
    }
}
// @lc code=end

#[test]
fn test(){
    let input1 = 1;
    let input2 = 22;
    let ans = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 11, 12, 15, 22];
    let res = Solution::self_dividing_numbers(input1, input2);
    assert_eq!(res,ans);
}