use super::Solution;

/*
 * @lc app=leetcode.cn id=1122 lang=rust
 *
 * [1122] 数组的相对排序
 */

// @lc code=start
impl Solution {
    pub fn relative_sort_array_1(arr1: Vec<i32>, arr2: Vec<i32>) -> Vec<i32> {
        let mut arr1 = arr1.clone();
        let mut hash = std::collections::HashMap::new();
        for (idx,&val) in arr2.iter().enumerate(){
            hash.insert(val, idx);
        }

        arr1.sort_by(|a,b|{
            let idxa = hash.get(a);
            let idxb = hash.get(b);
            if let Some(ia) = idxa{
                if let Some(ib) = idxb{
                    return ia.cmp(ib);
                }else{
                    return std::cmp::Ordering::Less;
                }
            }else if let Some(ib) = idxb{
                return std::cmp::Ordering::Greater;
            }else{
                return a.cmp(b);
            }
        });

        return arr1;
    }

    pub fn relative_sort_array(arr1: Vec<i32>, arr2: Vec<i32>) -> Vec<i32> {
        let mut ans = vec![];

        let mut count_list = vec![0;1001];
        for i in arr1{
            count_list[i as usize] +=1;
        }

        for i in arr2{
            while count_list[i as usize]>0{
                ans.push(i);
                count_list[i as usize]-=1;
            }
        }
        for (idx,count) in count_list.iter_mut().enumerate(){
            while *count > 0{
                ans.push(idx as i32);
                *count -=1;
            }
        }

        return ans;
    }
}
// @lc code=end

