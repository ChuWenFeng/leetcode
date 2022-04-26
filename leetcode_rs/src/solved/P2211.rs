use super::Solution;
/*
 * @lc app=leetcode.cn id=2211 lang=rust
 *
 * [2211] 统计道路上的碰撞次数
 */

// @lc code=start
//"LLLRSSRSRRRLRRLLSSLSLLRLLSRRRRRRSLLRRLLLSRRSSSS"
impl Solution {
    pub fn count_collisions(directions: String) -> i32 {
        let mut directions:Vec<char> = directions.chars().collect();
        let mut pre_direct = directions[0];
        let mut ans = 0;
        let mut r_count = 0;
        for &c in directions.iter().skip(1){
            match c{
                'L'=>{
                    if pre_direct == 'R'{
                        ans+=2;
                        ans+= r_count;
                        r_count = 0;
                        pre_direct = 'S';
                    }else if pre_direct == 'S'{
                        ans+=1;
                        pre_direct = 'S';
                    }
                },
                'R'=>{
                    if pre_direct == 'R'{
                        r_count += 1;
                    }
                    pre_direct = 'R';
                },
                'S'=>{
                    if pre_direct == 'R'{
                        ans+=r_count+1;
                        r_count = 0;
                    }
                    pre_direct = 'S';
                },
                _=>{}
            }
        }
        ans
    }
}
// @lc code=end

#[test]
fn test(){
    let directions = "LLLRSSRSRRRLRRLLSSLSLLRLLSRRRRRRSLLRRLLLSRRSSSS".to_string();
    let ans = 31;
    let res = Solution::count_collisions(directions);
    assert_eq!(res,ans);
}