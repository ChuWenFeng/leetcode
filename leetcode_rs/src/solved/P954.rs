use std::collections::HashMap;

use super::Solution;

/*
 * @lc app=leetcode.cn id=954 lang=rust
 *
 * [954] 二倍数对数组
 */

// @lc code=start
impl Solution {
    pub fn can_reorder_doubled(arr: Vec<i32>) -> bool {
        use std::collections::HashMap;
        let len = arr.len();
        let mut tab:HashMap<i32,i32> = HashMap::new();
        let mut ins:HashMap<i32,i32> = HashMap::new();
        for i in arr{
            if let Some(v)=tab.get_mut(&i){
                *v += 1;
            }else{
                tab.insert(i, 1);
            }
        }
        if let Some(v) = tab.get(&0) {
                if v % 2 != 0{
                    return false;
                }
        }
        let mut queue = std::collections::VecDeque::new();
        for (&val,(&num)) in tab.iter(){
            if val % 2 == 0{
                if let Some(&v) = tab.get(&(val/2)){
                    ins.insert(val, v);
                }else{
                    queue.push_back(val);
                }
            }else{
                queue.push_back(val);
            }
        }

        while let Some(node) = queue.pop_front(){
            let mut n1 = tab.get(&node).unwrap().clone();
            let n2 = if let Some(val) =  tab.get_mut(&(node*2)){
                val
            }else{
                return false;
            };

            if *n2 <n1{
                return false;
            }

           * n2 -= n1;

            let mut i2 =  ins.get_mut(&(node*2)).unwrap();
            *i2 -= n1;

            if *i2 == 0 && *n2 != 0{
                queue.push_back(node*2);
            }
            if let Some(i4) = ins.get_mut(&(node*4)){
                * i4 -= n1;
                if *i4 == 0 && *tab.get(&(node*4)).unwrap() != 0{
                    queue.push_back(node*4);
                }
            }
        }

        return true;
    }
}
// @lc code=end

#[test]
fn test(){
    let input = vec![4,-2,2,-4];
    let ans = true;

    let res = Solution::can_reorder_doubled(input);
    assert_eq!(res,ans);
}