use super::Solution;

/*
 * @lc app=leetcode.cn id=204 lang=rust
 *
 * [204] 计数质数
 */

// @lc code=start
impl Solution {
    pub fn count_primes_1(n: i32) -> i32 {
        let mut count = 1;
        if n <=2 {
            return 0;
        }
        for i in 3..n{
            let mut div = 2;
            let mut p_flag = true;
            while div * div <= i{
                if i % div ==0{
                    p_flag = false;
                    break;
                }
                div+=1;
            }
            if p_flag{
                count+=1;
            }
        }
        return count;
    }

    pub fn count_primes_2(n: i32) -> i32 {
        let n = n as usize;
        let mut count = 0;
        if n <=2 {
            return 0;
        }
        let mut primelist = vec![true;n];
        for i in 2..n{
            if primelist[i]{
                count+=1;
                let mut j = i*i;
                while j < n{
                    primelist[j] = false;
                    j+=i;
                }
            }
        }
        
        return count;
    }

    pub fn count_primes(n: i32) -> i32 {
        let n = n as usize;
        if n <=2 {
            return 0;
        }
        let mut primeflag = vec![true;n];
        let mut primes = vec![];
        
        for i in 2..n{
            if primeflag[i]{
                primes.push(i);
            }

            for &prime in primes.iter(){
                if i*prime >= n{
                    break;
                }
                primeflag[i*prime] = false;
                if i%prime == 0{
                    break;
                }
            }
        }

        
        return primes.len() as i32;
    }

}
// @lc code=end

