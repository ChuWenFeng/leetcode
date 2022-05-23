use super::Solution;
/*
 * @lc app=leetcode.cn id=315 lang=rust
 *
 * [315] 计算右侧小于当前元素的个数
 */

// @lc code=start
impl Solution {
    pub fn count_smaller(nums: Vec<i32>) -> Vec<i32> {
        let len = nums.len();
        let mut res = vec![0;nums.len()];
        let mut temp:Vec<usize> =  vec![0;nums.len()];
        let mut index:Vec<usize> = (0..nums.len()).collect();

        mergeAndCount(&nums, 0, len-1, &mut index, &mut temp, &mut res);

        res
    }
}

fn mergeAndCount(nums:&Vec<i32>,left:usize,right:usize,index:&mut Vec<usize>,temp:&mut Vec<usize>,res:&mut Vec<i32>){
    if left == right{
        return;
    }
    let mid = left+(right-left)/2;
    mergeAndCount(nums, left, mid, index, temp, res);
    mergeAndCount(nums, mid+1, right, index, temp, res);
    if nums[index[mid]] <= nums[index[mid+1]]{
        return;
    }
    mergeTwoSortList(nums, left, mid, right, index, temp, res);
}

fn mergeTwoSortList(nums:&Vec<i32>,left:usize,mid:usize,right:usize,index:&mut Vec<usize>,temp:&mut Vec<usize>,res:&mut Vec<i32>){
    for i in left..=right{
        temp[i] = index[i];
    }
    let mut i = left;
    let mut j = mid+1;

    for k in left..=right{
        if i > mid{
            index[k] = temp[j] as usize;
            j+=1;
        }else if j > right{
            index[k] = temp[i] as usize;
            i+=1;
            res[index[k]] += (right-mid) as i32;
        }else if nums[temp[i]] <= nums[temp[j]]{
            index[k] = temp[i];
            i+=1;
            res[index[k]]+=(j-mid-1) as i32;
        }else{
            index[k] = temp[j];
            j+=1;
        }
    }

}



// @lc code=end

