use super::Solution;
/*
 * @lc app=leetcode.cn id=321 lang=rust
 *
 * [321] 拼接最大数
 */

// @lc code=start
use std::cmp::Ordering;

impl Solution {
    pub fn max_number(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> Vec<i32> {
        let mut ans = vec![];
        let mut k = k as usize;
        for i in 0..=k{
            let v1 = max_sub_number(&nums1, i);
            let v2 = max_sub_number(&nums2, k-i);
            if v1.is_some() && v2.is_some(){
                let merge_max = merge_max_number(v1.unwrap(), v2.unwrap());
                ans = max_number(merge_max, ans);
            }
        }
        ans
    }
}


fn max_sub_number(num:&Vec<i32>,k:usize)->Option<Vec<i32>>{
    let mut stack = vec![];
    if k > num.len(){
        return None;
    }
    let mut k = num.len()-k;
    for &v in num.iter(){
        while  k>0 && !stack.is_empty() && v > *stack.last().unwrap(){
            stack.pop();
            k-=1;
        }
        stack.push(v);
    }

    while k>0{
        stack.pop();
        k-=1;
    }

    Some(stack)
}

fn lexicographicalLess(a:&[i32], b:&[i32]) ->bool {
    let mut i = 0;
    while i < a.len() && i < b.len(){
        if a[i] != b[i] {
            return a[i] < b[i]
        }
        i+=1;
    }
    return a.len() < b.len();
}

fn merge_max_number(v1:Vec<i32>,v2:Vec<i32>)->Vec<i32>{
    let mut l1 = 0;
    let mut l2 = 0;
    let mut res = vec![0;v1.len()+v2.len()];
    let mut j = 0;
    let mut k = 0;
    for i in 0..res.len(){
        if lexicographicalLess(&v1[j..], &v2[k..]){
            res[i] = v2[k];
            k+=1;
        }else{
            res[i] = v1[j];
            j+=1;
        }
    }

    res
}

fn max_number(num1:Vec<i32>,num2:Vec<i32>)->Vec<i32>{
    if num1.len() == 0{
        return num2;
    }
    if num2.len() == 0{
        return num1;
    }

    for i in 0..num1.len(){
        if num1[i]>num2[i]{
            return num1;
        }else if num1[i]<num2[i]{
            return num2;
        }
    }
    return num1;
}

// @lc code=end

#[test]
fn test(){
    let nums1 = vec![2,5,6,4,4,0];
    let nums2 = vec![7,3,8,0,6,5,7,6,2];
    let k = 15;
    let ans = vec![7,3,8,2,5,6,4,4,0,6,5,7,6,2,0];
    let res = Solution::max_number(nums1, nums2, k);
    assert_eq!(res,ans);

}