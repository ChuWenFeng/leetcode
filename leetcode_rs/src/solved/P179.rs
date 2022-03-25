use std::cmp::Ordering;

use super::Solution;

/*
 * @lc app=leetcode.cn id=179 lang=rust
 *
 * [179] 最大数
 */

// @lc code=start
impl Solution {
    pub fn largest_number(nums: Vec<i32>) -> String {
        use std::cmp::Ordering;
        let mut ans = String::new();
        let mut nums:Vec<u64> = nums.iter().map(|&x|x as u64).collect();
        nums.sort_by(|&a,&b|{
            let (mut ax,mut bx) = (10,10);
            while ax<=a{
                ax*=10;
            }
            while bx<=b{
                bx*=10;
            }

            (ax*b+a).cmp(&(bx*a+b))  
            
        });

        let mut flag = true;

        for i in nums{
            if flag && i == 0{
                continue;
            }else{
                flag = false;
            }
            ans = format!("{}{}",ans,i);
        }

        if ans == "".to_string(){
            return "0".to_string()
        }
        return ans;
    }
}
// @lc code=end

#[test]
fn test(){
    let input = vec![0,0];
    let ans = "0".to_string();
    let res = Solution::largest_number(input);
    assert_eq!(res,ans);
}