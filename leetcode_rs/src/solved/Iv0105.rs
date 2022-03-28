use super::Solution;


impl Solution {
    pub fn one_edit_away(first: String, second: String) -> bool {

        let l1 = first.len();
        let l2 = second.len();
        let mut df = 0;
        df = if l1 > l2 {l1-l2}else{l2-l1};
        if df >= 2{
            return false;
        }

        let first:Vec<char> = first.chars().collect();
        let second:Vec<char> = second.chars().collect();
        let mut differ = false;

        if df == 0{
            for i in 0..l1{
                if first[i] != second[i] {
                    if !differ{
                        differ = true;
                    }else{
                        return false;
                    }
                }
            }
        }else{
            let mut i = 0;
            let mut j = 0;
            while i<l1 && j<l2 {
                if first[i] != second[j]{
                    if !differ{
                        differ = true;
                    }else{
                        return false;
                    }
                    if l1 > l2{i+=1;}else{j+=1;}
                }else{
                    i+=1;
                    j+=1;
                }
            }
        }
        return true;
    }
}