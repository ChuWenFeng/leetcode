use super::Solution;
/*
 * @lc app=leetcode.cn id=327 lang=rust
 *
 * [327] 区间和的个数
 */

// @lc code=start
impl Solution {
    pub fn count_range_sum(nums:Vec<i32>, lower: i32, upper: i32) -> i32 {
        let mut prefixSum:Vec<i64> = vec![0;nums.len()+1];
        for (i,&v) in nums.iter().enumerate(){
            prefixSum[i+1] = prefixSum[i]+v as i64;
        }
        return mergeCount(&mut prefixSum, lower as i64, upper as i64) as i32;
        
    }
}

fn mergeCount(arr:&mut Vec<i64>,lower:i64,upper:i64)->i64{
    let n = arr.len();
    if n <= 1{
        return 0;
    }
    let mut n1:Vec<i64> = arr[..n/2].iter().map(|x|*x).collect();
    let mut n2:Vec<i64> = arr[n/2..].iter().map(|x|*x).collect();
    let mut cnt=mergeCount(&mut n1,lower,upper)+mergeCount(&mut n2,lower,upper);
    let mut l = 0;
    let mut r = 0;
    for v in n1.iter(){
        while l < n2.len() && n2[l]-v < lower{
            l+=1;
        }
        while r < n2.len() && n2[r]-v <= upper{
            r+=1;
        }
        cnt+=(r-l) as i64;
    }

    let mut p1 = 0;
    let mut p2= 0;
    for i in 0..n{
        if p1 < n1.len() && (p2 == n2.len() || n1[p1]<=n2[p2]){
            arr[i]=n1[p1];
            p1+=1;
        }else{
            arr[i]=n2[p2];
            p2+=1;
        }
    }
    return cnt;
}

// @lc code=end

