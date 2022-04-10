
use super::Solution;

/*
 * @lc app=leetcode.cn id=215 lang=rust
 *
 * [215] 数组中的第K个最大元素
 */

// @lc code=start
impl Solution {
    pub fn find_kth_largest_1(nums: Vec<i32>, k: i32) -> i32 {
        let mut idx = 0;
        let mut k = k;
        let mut left = 0;
        let mut right = nums.len()-1;
        let target = nums.len() - k as usize;
        let mut nums = nums.clone();
        idx = fastSort(&mut nums, left, right);
        while idx != target{
            if idx < target{
                left = idx+1;
            }else{
                right = idx-1;
            }
            idx = fastSort(&mut nums, left, right);
        }

        return nums[idx];
    }

    pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
        let mut nums = nums.clone();
        let k = k as usize;
        for i in 0..k{
            swim(&mut nums, i);
        }

        for i in k..nums.len(){
            if nums[i] > nums[0]{
                let tmp = nums[i];
                nums[i] = nums[0];
                nums[0] = tmp;

                sink(&mut nums, 0, k-1);
            }

        }

        return nums[0];
    }
}

fn swim(nums:&mut Vec<i32>,mut idx:usize){
    while idx > 0 && nums[idx]<nums[(idx-1)/2]{
        let tmp = nums[idx];
        nums[idx] = nums[(idx-1)/2];
        nums[(idx-1)/2] = tmp;
        idx = (idx-1)/2;
    }
}

fn sink(nums:&mut Vec<i32>,mut idx:usize,limit:usize){
    while 2*idx+1<=limit{
        let mut ch = 2*idx+1;
        if ch+1<limit && nums[ch+1]<nums[ch]{
            ch+=1;
        }
        if nums[idx]<nums[ch]{
            break;
        }
        let tmp = nums[idx];
        nums[idx] = nums[ch];
        nums[ch] = tmp;

        idx = ch;
    }
}

fn fastSort(nums:&mut Vec<i32>,mut left:usize,mut right:usize)->usize{
    if left >= right{
        return left;
    }
    let a = nums[left];
    let l = left;
    let r = right;
    while left < right{
        while nums[right]>a && left < right{
            right-=1;
        }
        while nums[left]<=a && left < right{
            left+=1;
        }
        if left < right{
            let tmp = nums[left];
            nums[left] = nums[right];
            nums[right] = tmp;
        }
    }
    nums[l] = nums[left];
    nums[left] = a;

    // if l < left{
    //     fastSort(nums, l, left-1);
    // }
    // if r > left{
    //     fastSort(nums, left+1, r);
    // }

    return left;
}



// @lc code=end

#[test]
fn test(){
    let mut input = vec![3,2,1,5,6,4];
    let len = input.len();
    let res = fastSort(&mut input, 0, len-1);
    dbg!(res);
    dbg!(input);
}