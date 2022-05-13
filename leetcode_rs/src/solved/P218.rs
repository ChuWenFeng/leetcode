

use super::Solution;
/*
 * @lc app=leetcode.cn id=218 lang=rust
 *
 * [218] 天际线问题
 */

// @lc code=start
use std::cmp::{Ordering,Eq};
use std::collections::{BinaryHeap, VecDeque};
use std::fmt::Binary;


#[derive(Eq)]
struct pair{
    right:i32,
    height:i32,
}

impl pair{
    pub fn new(right:i32,height:i32)->pair{
        pair { right, height }
    }
}

impl PartialEq for pair{
    fn eq(&self, other: &Self) -> bool {
        self.height == other.height && self.right == other.right
    }
}

impl Ord for pair{
    fn cmp(&self, other: &Self) -> Ordering {
        self.height.cmp(&other.height)
    }
}
impl PartialOrd for pair {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Solution {
    pub fn get_skyline(buildings: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut boundaries :Vec<i32> = vec![];
        buildings.iter().map(|x|{
            boundaries.push(x[0]);
            boundaries.push(x[1]);
        }).count();

        boundaries.sort();

        let mut h:BinaryHeap<pair> = BinaryHeap::new();
        let mut ans:Vec<Vec<i32>> = vec![];
        let n = buildings.len();
        let mut idx = 0;
        for &boundary  in boundaries.iter(){
            while idx  < n && buildings[idx][0] <= boundary{
                h.push(pair::new(buildings[idx][1],buildings[idx][2]));
                idx+=1;
            }
            while h.len()>0 && h.peek().unwrap().right <= boundary{
                h.pop();
            }
            let mut max = 0;
            if h.len()>0{
                max = h.peek().unwrap().height;
            }
            if ans.len() == 0 || max != ans.last().unwrap()[1]{
                ans.push(vec![boundary,max]);
            }
        }

        ans
    }
}
// @lc code=end

#[test]
fn test(){
    let buildings:Vec<Vec<i32>> = [[2,9,10],[3,7,15],[5,12,12],[15,20,10],[19,24,8]].iter().map(|x|{
        let mut list:Vec<i32> = vec![];
        list.extend(x.iter());
        list
    }).collect();
    let ans:Vec<Vec<i32>> = [[2,10],[3,15],[7,12],[12,0],[15,10],[20,8],[24,0]].iter().map(|x|{
        let mut list:Vec<i32> = vec![];
        list.extend(x.iter());
        list
    }).collect();

    let res = Solution::get_skyline(buildings);
    assert_eq!(res,ans);

}