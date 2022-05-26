use super::Solution;
/*
 * @lc app=leetcode.cn id=324 lang=rust
 *
 * [324] 摆动排序 II
 */

// @lc code=start
impl Solution {
    pub fn wiggle_sort(nums: &mut Vec<i32>) {
        let mut mididx = nums.len()/2;
        let (_,&mut mid,_) = nums.select_nth_unstable(mididx);
        let mut i = 0;
        let mut j = 0;
        let mut k = nums.len()-1;
        while j<k {
            if nums[j] > mid{
                nums.swap(j, k);
                k-=1;
            }else if nums[j] < mid{
                nums.swap(j, i);
                i+=1;
                j+=1;
            }else{
                j+=1;
            }
        }
        if nums.len()%2 == 1{
            mididx+=1;
        }

        let tmp = nums.clone();
        for i in 0..mididx{
            nums[2*i] = tmp[mididx-1-i];
        }
        for i in 0..nums.len()-mididx{
            nums[2*i+1] = tmp[tmp.len()-1-i];
        }

    }
}
// @lc code=end
//[1,3,2,2,3,1]
#[test]
fn test(){
    let mut nums = vec![1,3,2,2,3,1];
    let ans = vec![1,6,1,5,1,4];
    Solution::wiggle_sort(&mut nums);
    assert_eq!(nums,ans);
}