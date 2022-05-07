use super::Solution;

impl Solution {
    pub fn keyboard(k: i32, n: i32) -> i32 {
        // dfs(26, n as i64, k as i64) as i32
        dp(k, n)
    }
}

fn dp(k:i32,n:i32)->i32{
    let n = n as usize;
    let k = k as usize;
    let mut dp = vec![vec![0;27];(n+1) as usize];
    for i in 0..=26{
        dp[0][i] = 1;
    }

    for i in 1..=n{
        for j in 1..=26{
            let min = i.min(k);
            for x in 0..=min{
                dp[i][j] += dp[i-x][j-1]*comb(x,i);
            }
            dp[i][j] %= 1000000007;
        }
    }


    return dp[n as usize][26] as i32
}

fn comb(n:usize,m:usize)->usize{
    let mut k =1;
    let mut ans = 1;
    while k <= n{
        ans = ((m-k+1)*ans)/k;
        k+=1;
    }
    return ans;
}

/// 超时
fn dfs(abnum:usize,n:usize,k:usize)->usize{

    if n == 0{
        return 1;
    }
    if abnum*k < n{
        return 0;
    }

    let mut ans = 0;
    let min = n.min(k);
    
    for i in 0..=min{
        ans += (dfs(abnum-1, n-i, k)*comb(i, n)) % 1000000007;
        
    }
    return ans % 1000000007;
}




#[test]
fn test(){
    let k = 5;
    let n = 50;
    let ans = 363766962;
    let res = Solution::keyboard(k, n);
    assert_eq!(res,ans);
}